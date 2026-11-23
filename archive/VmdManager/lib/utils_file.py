import os
import shutil
import re

from sqlalchemy import insert, and_, func
from sqlalchemy.orm import Session
from sqlalchemy.exc import NoResultFound
from models import File, Song, Singer, SongSinger, SongFile, Author, AuthorFile, Tag, FileTag

import vmdlib
import utils
from utils import calculate_hash

'''功能：扫描文件加入数据库'''
def scan_directory_and_add_to_db(session, directory, FileDatabasePath, folder_format, song_name_format, update):
    utils.log_and_print(f'Scaning {directory}')
    # 分类文件，分为存在除txt文件以外文件的有用文件合集（文件夹），以及零散的txt文件（主要是playlist）
    # 逻辑是当前文件夹内是否存在txt文件以外的文件
    for root, dirs, files in os.walk(directory):
        if files:
            # 通过文件位置获取作者，日期和歌曲名
            result= process_path(root, FileDatabasePath, folder_format, song_name_format)
            author = result.get('author', None)
            date = result.get('date', None)
            song_name = result.get('song', None)
            singer_name = result.get('singer', None)
            tag = result.get('tag', None)
            if song_name:
                if not author:
                    author = '未知作者'
                if not date:
                    date = '未知时间'
                if not singer_name:
                    singer_name = '未知歌手'

                singer_names = [singer_name]
                # 插入歌曲和歌手信息
                song_id, singer_ids = add_song_and_singers_to_db(session, song_name, singer_names)

            if author: 
                # 插入作者表
                author_instance = session.query(Author).filter_by(name=author).first()
                if not author_instance:
                    author_instance = Author(name=author, description='无描述')
                    session.add(author_instance)
                    session.flush()  # 刷新以获取 author_id'

            if tag:
                tag_instance = session.query(Tag).filter_by(name=tag).first()
                if not tag_instance:
                    tag_instance = Tag(name=tag)
                    session.add(tag_instance)
                    session.flush()  # 刷新以获取 author_id'

            for file in files:
                file_instance = add_file_to_db(session, file, root, date, FileDatabasePath, update)
                if song_name:
                    file_instance.song_files.clear()
                    # 创建歌曲-文件关联
                    song_file = SongFile(song_id=song_id, file_id=file_instance.id)
                    session.merge(song_file)

                if author: 
                    file_instance.author_files.clear()
                    # 创建作者-文件关联
                    author_file = AuthorFile(author_id=author_instance.id, file_id=file_instance.id)
                    session.merge(author_file)
                    # 创建tag-文件关联
                    
                if tag:
                    file_tag = FileTag(file_id=file_instance.id, tag_id=tag_instance.id)
                    session.merge(file_tag)

def add_file_to_db(session: Session, file, root, date, FileDatabasePath, update):
    root = os.path.normpath(root)
    file_name = file
    file_folder_path = os.path.relpath(root, start=FileDatabasePath)
    
    # 检查文件名和文件夹路径的组合是否已经存在
    existing_file = session.query(File).filter(
        and_(File.name == file_name, File.folder_path == file_folder_path)
    ).first()
    
    if existing_file:
        if update:
            utils.log_and_print(f"File {file_name} in folder {root} already exists. Updating record.")
        else:
            utils.log_and_print(f"File {file_name} in folder {root} already exists in the database. Skipping.")
            return
    else:
        utils.log_and_print(f"Inserting new file {file_name} into the database.")
    
    file_path = os.path.join(root, file)
    file_type = os.path.splitext(file)[-1].lower().lstrip('.')
    file_size = os.path.getsize(file_path)
    file_hash = calculate_hash(file_path)  # 假设你有计算文件哈希值的函数

    file_data = {
        'name': file_name,
        'folder_path': file_folder_path,
        'type': file_type,
        'vmd_type': 'notvmd',
        'size': file_size,
        'hash': file_hash,
        'date': date,
        'source': '未知来源',
        'char_name': 'notvmd',
        'bone_key_frame_num': 0,
        'morph_key_frame_num': 0,
        'camera_key_frame_num': 0,
        'light_key_frame_num': 0,
        'self_shadow_frame_num': 0,
        'ik_key_frame_num': 0
    }
    # 处理vmd数据
    if file_type == 'vmd':
        result = get_vmd_info(file_path, file_data)
        if result:
            file_data = result

    # 插入或更新文件    
    file_id = upsert_file(session, file_data, existing_file, verify_exists = False)
 
    return file_id

# 处理vmd数据
def get_vmd_info(file_path, file_data):
    vmd_types = [] 

    result = vmdlib.check_vmd(file_path, file_data)
    if len(result) == 2:
        vmd_types, file_data = result
        file_data['vmd_type'] = ','.join(vmd_types) 
    else:
        utils.log_and_print(f"Error: VMD file check failed or incomplete data returned for:{file_path}")
        return None
    return file_data

def upsert_file(session: Session, file_data: dict, existing_file=None, verify_exists: bool = True):
    """
    Update an existing file record or insert a new file record in the database.
    :param session: SQLAlchemy session object.
    :param file_data: Dictionary containing file attributes.
    :param verify_exists: Boolean indicating whether to verify file existence in the database.
    :return: file_id of the updated or inserted file.
    """  
    if verify_exists:
        existing_file = session.query(File).filter(File.name == file_data['name'], File.folder_path == file_data['folder_path']).first()
    
    if existing_file:
        # Update existing file record
        for key, value in file_data.items():
            setattr(existing_file, key, value)  # Dynamically assign attributes
        # Clear relationships if necessary
        session.flush()  # Flush to retrieve file_id
        return existing_file
    else:
        # Insert a new file record
        new_file = File(**file_data)
        session.add(new_file)
        session.flush()  # Flush to retrieve file_id
        return new_file

def add_song_and_singers_to_db(session: Session, song_name: str, singer_names=['未知歌手']):
    # 第一步：处理每个歌手
    singer_ids = []
    song = None
    song_singer_may_exist = True
    for singer_name in singer_names:
        try:
            # 尝试从数据库中检索歌手
            singer = session.query(Singer).filter_by(name=singer_name).one()
            utils.log_and_print(f"歌手 {singer_name} 已存在数据库中。")
        except NoResultFound:
            # 如果歌手不存在，则创建新的歌手记录
            utils.log_and_print(f"将新歌手 {singer_name} 插入数据库中。")
            singer = Singer(name=singer_name)
            session.add(singer)
            session.flush()  # 确保 singer_id 已经生成
            song_singer_may_exist = False
        singer_ids.append(singer.id)

    # 第二步：查找具有相同歌手组合的现有歌曲
    if song_singer_may_exist:
        song = find_song_by_name_and_singers(session, song_name, singer_ids)
    if not song or not song_singer_may_exist:
        # 如果歌曲不存在，则创建新的歌曲记录
        utils.log_and_print(f"将新歌曲 {song_name} 插入数据库中。")
        song = Song(name=song_name, description='无描述')
        session.add(song)
        session.flush()  # 确保 song_id 已经生成
    else:
        return song.id, singer_ids

    # 第三步：将歌曲与歌手关联
    for singer_id in singer_ids:
        link_exists = session.query(SongSinger).filter_by(song_id=song.id, singer_id=singer_id).first()
        if not link_exists:
            utils.log_and_print(f"将歌曲 {song_name} 与歌手 ID {singer_id} 关联。")
            song_singer_link = SongSinger(song_id=song.id, singer_id=singer_id)
            session.add(song_singer_link)
        else:
            utils.log_and_print(f"歌曲 {song_name} 和歌手 ID {singer_id} 已经关联。")
    
    return song.id, singer_ids

def find_song_by_name_and_singers(session: Session, song_name, singer_ids):
    # Step 1: 查询所有与给定 song_name 匹配的歌曲
    songs = session.query(Song).filter(Song.name == song_name).all()

    # Step 2: 遍历每一首歌曲，查询其关联的所有歌手
    for song in songs:
        # 获取这首歌的所有关联歌手 ID
        associated_singers = session.query(SongSinger.singer_id).filter(SongSinger.song_id == song.id).all()
        associated_singer_ids = [singer_id for singer_id, in associated_singers]
        
        # Step 3: 比较歌手列表
        if set(associated_singer_ids) == set(singer_ids):
            # 如果关联的歌手列表与给定的歌手列表相同，返回这首歌
            utils.log_and_print(f"找到现有的歌曲 {song_name}，其歌手组合已匹配（歌曲ID：{song.id}）。")
            return song

    # 如果没有找到匹配的歌曲，返回 None
    return None

def process_path(root, base_directory, folder_format, song_name_format):
    # 获取相对于指定文件夹位置的相对路径
    relative_path = os.path.relpath(root, base_directory)
    result = parse_path(folder_format, relative_path)

    # 假定不存在未知曲目，所有演出文件至少都有一个song_name文件夹
    # song_name不一定指歌，也可能是类型之类的
    song_name = result.get('song', None)
    if song_name and song_name_format != '' and 'singer' not in folder_format:
        result = extract_song_info(result, song_name_format)

    return result

# 需要是相对路径
def parse_path(template, file_path):
    # 解析模板
    template_parts = template.split('-')
    path_parts = os.path.normpath(file_path).split(os.sep)

    # 检查是否有省略号，确定其位置
    ellipsis_index = None
    if '...' in template_parts:
        ellipsis_index = template_parts.index('...')
    
    result = {}
    ellipsis_values = []  # 存储省略部分的路径值
    j = 0  # 用于path_parts的索引
    for i, part in enumerate(template_parts):
        if j >= len(path_parts):
            # Break the loop if there are no more path parts to process
            break

        if part == '...':
            # 计算省略部分的结束索引
            ellipsis_end_index = len(path_parts) - (len(template_parts) - i - 1)
            # 收集省略部分的路径值
            ellipsis_values = path_parts[j:ellipsis_end_index]
            j = ellipsis_end_index  # 更新索引为省略部分后的第一个元素
        else:
            result[part] = path_parts[j]
            j += 1
    
    # 将省略部分的路径值存储到结果字典中
    if ellipsis_index is not None:
        result['ellipsis_parts'] = ellipsis_values
        print(ellipsis_values)

    return result

def extract_song_info(path_data, pattern_template):
    song_name = path_data['song']
    # 如果模板中包含歌手和歌名的占位符，尝试匹配
    if "singer" in pattern_template and "song" in pattern_template:
        pattern_string = pattern_template.replace("singer", "(?P<singer>.+?)").replace("song", "(?P<song>.+)")
        pattern = re.compile(pattern_string)
        match = pattern.search(song_name)
        if match:
            path_data['singer'] = match.group('singer')
            path_data['song'] = match.group('song')
    # 如果模板仅包含歌名占位符，假设整个字符串是歌曲名
    elif "song" in pattern_template:
        pattern_string = pattern_template.replace("song", "(?P<song>.+)")
        pattern = re.compile(pattern_string)
        match = pattern.search(song_name)
        if match:
            path_data['song'] = match.group('song')
    # 如果没有匹配到任何模式，返回默认值
    return path_data

'''功能：扫描数据库外部文件删除与数据库相同的部分'''
def scan_folder_delete_duplicate(session, directory):
    utils.log_and_print(f'Scaning {directory}')
    for root, dirs, files in os.walk(directory):
        for file in files:
            scan_file_delete_duplicate(session, file, root, directory)
    delete_empty_folders(directory)

def scan_file_delete_duplicate(session, file, root, directory):
    root = os.path.normpath(root)
    file_path = os.path.join(root, file)
    file_size = os.path.getsize(file_path)
    if file_size == 0:
        utils.log_and_print(f"File:{file_path} is 0 kb file, skipping.")
    file_hash = calculate_hash(file_path)
    file_exist = session.query(File).filter(File.hash == file_hash).first()
    if file_exist:
        file_exist_path = os.path.join(file_exist.folder_path, file_exist.name)
        # 如果存在相同哈希的文件，则移动当前文件到duplicate目录
        duplicate_directory = os.path.join(directory, 'duplicate')
        os.makedirs(duplicate_directory, exist_ok=True)  # 确保duplicate目录存在
        relative_path = os.path.relpath(file_path, start=directory)  # 获取文件相对于directory的路径
        new_path = os.path.join(duplicate_directory, relative_path)
        os.makedirs(os.path.dirname(new_path), exist_ok=True)  # 确保新路径的文件夹结构存在
        shutil.move(file_path, new_path)  # 移动文件
        utils.log_and_print(f"Duplicate File:{file_path} : {file_exist_path} moved to: {new_path}")
    else:
        utils.log_and_print(f"No duplicate found for: {file_path}")

'''功能：查找被移动的文件'''
def find_moved_files(session, directory, base_directory):
    utils.log_and_print(f'Scaning {directory}')
    for root, dirs, files in os.walk(directory):
        for file in files:
            find_moved_file(session, file, root, base_directory)

def find_moved_file(session, file, root, directory):
    root = os.path.normpath(root)
    file_path = os.path.join(root, file)
    file_rel_folder_path = os.path.relpath(root, directory)
    file_hash = calculate_hash(file_path)
    file_exists = session.query(File).filter(File.hash == file_hash).all()
    for file_exist in file_exists:
        file_exist_path = os.path.join(directory, file_exist.folder_path, file_exist.name)
        if not os.path.exists(file_exist_path):
            file_exist.folder_path = file_rel_folder_path
            file_exist.name = file
            session.flush()
            break

'''功能：删除空文件夹'''
def delete_empty_folders(directory):
    # 使用 os.walk 遍历目录，topdown=False 使得从子目录开始向上遍历
    for root, dirs, files in os.walk(directory, topdown=False):
        for dir in dirs:
            # 构造完整的目录路径
            full_path = os.path.join(root, dir)
            full_path = os.path.normpath(full_path)
            # 尝试删除目录，如果目录为空则成功删除
            try:
                os.rmdir(full_path)
                utils.log_and_print(f"Deleted empty folder: {full_path}")
            except OSError as e:
                # 如果目录非空，会抛出 OSError
                utils.log_and_print(f"Failed to delete {full_path}: {e}")

'''功能：重整文件'''
def rearrange_files(session, directory, rearrange_format, origin_format, FileDatabasePath):
    utils.log_and_print(f'rearranging {directory}')
    directory = os.path.normpath(directory)
    print(directory)
    print(FileDatabasePath)
    files = session.query(File).filter((FileDatabasePath + '\\' + File.folder_path).contains(directory)).all()
    for file in files: 
        move_and_update_file(session, file, rearrange_format, origin_format, FileDatabasePath)
    delete_empty_folders(directory)

    session.commit()

def move_and_update_file(session, file, rearrange_format, origin_format, FileDatabasePath):
    folder_data = parse_path(origin_format, file.folder_path)
    db_folder_data = get_path_info_from_db(session, file, folder_data)
    new_path = create_path_from_template(rearrange_format, db_folder_data)
    new_abs_path = os.path.join(FileDatabasePath, new_path)
    
    # 创建新文件夹（如果不存在）
    if not os.path.exists(new_abs_path):
        os.makedirs(new_abs_path)

    # 构建旧的文件路径和新的文件路径
    old_file_path = os.path.join(FileDatabasePath, file.folder_path, file.name)
    new_file_path = os.path.join(new_abs_path, file.name)

    # 检测到已有文件时，自动退出
    if os.path.exists(old_file_path):
        if os.path.exists(new_file_path):
            utils.log_and_print(f"Error: Target file already exists at {new_file_path}.")
            raise Exception("Target file already exists. Aborting operation.")  # 抛出异常
        else:
            shutil.move(old_file_path, new_file_path)
            utils.log_and_print(f"Moved: {old_file_path} to {new_file_path}")
    else:
        utils.log_and_print(f"File not found: {old_file_path}. Skipping.")

    # 更新数据库中的文件夹路径
    file.folder_path = new_path

# 从新模板和所得数据创建路径
def create_path_from_template(template, data):
    # 解析模板
    template_parts = template.split('-')
    
    # 创建路径的部分列表
    path_parts = []
    
    for part in template_parts:
        if part == '...':
            # 如果是省略号，添加省略部分的所有元素
            if 'ellipsis_parts' in data:
                path_parts.extend(data['ellipsis_parts'])
        elif part in data:
            # 如果键存在于数据中，添加其对应的值
            path_parts.append(data[part])
    
    # 使用系统特定的路径分隔符来构建完整路径
    return os.path.join(*path_parts)

# 从数据库获得信息
def get_path_info_from_db(session, file, origin_data):
    try:
        author = session.query(Author).join(
            AuthorFile, AuthorFile.author_id == Author.id
        ).filter(
            AuthorFile.file_id == file.id
        ).one()
    except NoResultFound:
        author = None

    try:
        song = session.query(Song).join(
            SongFile, SongFile.song_id == Song.id
        ).filter(
            SongFile.file_id == file.id
        ).one()
    except NoResultFound:
        song = None

    if song:
        try:
            singer = session.query(Singer).join(
                SongSinger, SongSinger.singer_id == Singer.id
            ).filter(
                SongSinger.song_id == song.id
            ).first()  # 这里仍然使用.first()因为我们只需要第一个匹配项
        except NoResultFound:
            singer = None
    else:
        singer = None

    data = {}

    if author:
        data['author'] = author.name
    if song:
        data['song'] = song.name
    if singer:
        data['singer'] = singer.name
    data['date'] = file.date

    if origin_data.get('ellipsis_parts') is not None:
        data['ellipsis_parts'] = origin_data['ellipsis_parts']
    
    return data

'''功能：删除文件不存在的file条目'''
def handle_deleted_file(session, FileDatabasePath):
    file_set = preload_file_paths(FileDatabasePath)
    try:
        # 使用分页查询以减少内存使用
        offset = 0
        limit = 100  # 每批次处理100个文件
        while True:
            files = session.query(File).offset(offset).limit(limit).all()
            if not files:
                break
            for file in files:
                file_path = os.path.join(FileDatabasePath, file.folder_path, file.name)
                if file_path not in file_set:
                    utils.log_and_print(f"Deleting record for missing file: {file_path}")
                    session.delete(file)
            session.commit()
            offset += limit
    except Exception as e:
        session.rollback()
        utils.log_and_print(f"Error handling file deletion: {e}")

def preload_file_paths(directory):
    file_paths = set()
    # 遍历目录下的所有文件和子目录
    for root, dirs, files in os.walk(directory):
        for file in files:
            # 构建完整的文件路径
            full_path = os.path.join(root, file)
            file_paths.add(full_path)
    return file_paths

'''
无用函数
'''
# 查询文件是否是playlist
def handle_playlist(file_path):
    expected_header = '; MMDDirector playlist'
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            header = file.readline().strip()
            if header == expected_header:
                utils.log_and_print(f"File format is correct. Header '{header}' found.")
                return True
            else:
                utils.log_and_print(f"Unexpected file format. Expected header '{expected_header}', but found '{header}'")
    except (UnicodeDecodeError, FileNotFoundError):
        utils.log_and_print(f"Failed to process file {file_path} due to encoding error or file not found.")
    return False

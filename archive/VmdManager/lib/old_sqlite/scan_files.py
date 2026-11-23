import os
import sqlite3
import blake3
import configparser

import vmdlib

# 读取配置文件
config = configparser.ConfigParser()
config.read('../config.ini')

# 获取路径信息
db_path = config['Paths']['db_path']
base_directory = config['Paths']['base_directory']

def calculate_hash(file_path):
    # 初始化 BLAKE3 哈希对象
    hash_func = blake3.blake3()

    # 打开文件并逐块读取内容
    with open(file_path, 'rb') as f:
        # 每次读取 8192 字节的内容
        while chunk := f.read(8192):
            # 更新哈希值，传入当前读取的块
            hash_func.update(chunk)

    # 返回完整文件的 BLAKE3 哈希值
    return hash_func.hexdigest()

def process_path(root, base_directory):
    # 获取相对于指定数据库文件夹位置的相对路径
    relative_path = os.path.relpath(root, base_directory)

    # 将相对路径拆分为各个部分
    path_parts = relative_path.split(os.sep) 
    
    # 初始化默认值
    author = '未知作者'
    date = '未知时间'
    song_name = '未知曲目'
    file_folder_middle_path = ""

    # 根据路径部分的长度分配值
    if len(path_parts) >= 3:
        author = path_parts[0]
        date = path_parts[1]
        song_name = path_parts[-1]
        # 提取中间层级路径部分
        if len(path_parts) > 3:
            file_folder_middle_path = os.path.join(*path_parts[2:-1])
    elif len(path_parts) == 2:
        author = path_parts[0]
        song_name = path_parts[1]
    elif len(path_parts) == 1:
        song_name = path_parts[0]
    
    return author, date, song_name, file_folder_middle_path

def insert_into_table(cursor, table_name, data, check_duplicate=False, duplicate_columns=None):
    """
    通用插入函数，支持检查重复记录。
    
    参数:
    - cursor: 数据库游标。
    - table_name: 要插入数据的表名。
    - data: 要插入的数据字典。
    - check_duplicate: 是否在插入前检查重复记录，默认为 False。
    - duplicate_columns: 如果检查重复，则提供要检查的列名列表。
    """
    if check_duplicate and duplicate_columns:
        # 构建重复检查的 WHERE 子句
        where_clause = ' AND '.join([f"{col} = ?" for col in duplicate_columns])
        where_values = [data[col] for col in duplicate_columns]
        
        # 检查是否存在重复记录
        cursor.execute(f'SELECT 1 FROM {table_name} WHERE {where_clause}', where_values)
        duplicate_exists = cursor.fetchone()
        
        if duplicate_exists:
            print(f"Duplicate entry found in table '{table_name}' for {duplicate_columns}. Skipping insertion.")
            return  # 如果存在重复记录，直接返回
    
    # 执行插入操作
    keys = ', '.join(data.keys())
    question_marks = ', '.join(['?'] * len(data))
    values = tuple(data.values())

    sql = f'INSERT INTO {table_name} ({keys}) VALUES ({question_marks})'
    cursor.execute(sql, values)
    print(f"Inserted data into table '{table_name}'.")

def update_table(cursor, table_name, data, where_clause):
    set_clause = ', '.join([f"{key} = ?" for key in data.keys()])
    values = list(data.values())
    
    where_keys = ' AND '.join([f"{key} = ?" for key in where_clause.keys()])
    where_values = list(where_clause.values())
    
    sql = f'UPDATE {table_name} SET {set_clause} WHERE {where_keys}'
    cursor.execute(sql, values + where_values)

def delete_from_table(cursor, table_name, where_clause):
    where_keys = ' AND '.join([f"{key} = ?" for key in where_clause.keys()])
    where_values = list(where_clause.values())

    sql = f'DELETE FROM {table_name} WHERE {where_keys}'
    cursor.execute(sql, where_values)

def add_song_and_singers_to_db(cursor, song_name, singer_names=['unknown']):
    # Step 1: 处理每个歌手
    singer_ids = []
    for singer_name in singer_names:
        cursor.execute('SELECT singer_id FROM singers WHERE name = ?', (singer_name,))
        singer_row = cursor.fetchone()

        if not singer_row:
            print(f"Inserting new Singer {singer_name} into the database.")
            singer_data = {
                'name' = singer_name
            }
            insert_into_table(cursor, 'singers', singer_data)
            singer_id = cursor.lastrowid  # 获取新插入歌手的 ID
        else:
            singer_id = singer_row[0]
            print(f"Singer {singer_name} already exists in the database.")
        
        singer_ids.append(singer_id)

    # Step 2: 查找现有的歌曲和歌手组合
    song_id = None
    cursor.execute('''
        SELECT ss.song_id FROM song_singers ss
        JOIN songs s ON ss.song_id = s.song_id
        WHERE s.name = ? AND ss.singer_id IN ({})
        GROUP BY ss.song_id
        HAVING COUNT(ss.singer_id) = ?
    '''.format(','.join(['?']*len(singer_ids))), [song_name] + singer_ids + [len(singer_ids)])
    song_row = cursor.fetchone()

    if song_row:
        song_id = song_row[0]
        print(f"Found existing Song {song_name} with matching singers (Song ID: {song_id}).")
    else:
        print(f"Inserting new Song {song_name} into the database.")
        song_description = 'unknown'
        cursor.execute('INSERT INTO songs (name, description) VALUES (?, ?)', (song_name, song_description))
        song_id = cursor.lastrowid  # 获取新插入歌曲的 ID

    # Step 3: 处理歌曲和歌手的关系
    for singer_id in singer_ids:
        cursor.execute('SELECT 1 FROM song_singers WHERE song_id = ? AND singer_id = ?', (song_id, singer_id))
        relationship_row = cursor.fetchone()

        if not relationship_row:
            print(f"Linking Song {song_name} with Singer ID {singer_id}.")
            cursor.execute('INSERT INTO song_singers (song_id, singer_id) VALUES (?, ?)', (song_id, singer_id))
        else:
            print(f"Song {song_name} and Singer ID {singer_id} are already linked.")
    return song_id, singer_ids

def add_file_to_db(cursor, file, root, update=False):
    file_name = file
    file_folder_path = root
    
    # 检查文件名和文件夹路径的组合是否已经存在
    cursor.execute('SELECT 1 FROM files WHERE file_name = ? AND file_folder_path = ?', (file_name, file_folder_path))
    row = cursor.fetchone()
    if row:
        if update:
            print(f"File {file_name} in folder {file_folder_path} already exists. Updating record.")
        else:
            print(f"File {file_name} in folder {file_folder_path} already exists in the database. Skipping.")
            return
    else:
        if not update:
            print(f"Inserting new file {file_name} into the database.")
    
    file_path = os.path.join(root, file)
    file_size = os.path.getsize(file_path)
    # 计算文件哈希值
    file_hash = calculate_hash(file_path)
    # 获得文件类型
    file_type = os.path.splitext(file)[-1].lower().lstrip('.')
    
    # 处理vmd数据
    vmd_types = []
    vmd_type_str = None
    charName = 'notvmd'
    boneKeyFrameNum = 0
    morphKeyFrameNum = 0
    cameraKeyFrameNum = 0
    lightKeyFrameNum = 0
    selfShadowFrameNum = 0
    IKKeyFrameNum = 0
    if file_type == 'vmd':
        vmd_types, charName, boneKeyFrameNum, morphKeyFrameNum, cameraKeyFrameNum, lightKeyFrameNum, selfShadowFrameNum, IKKeyFrameNum = vmdlib.checkVmd(file_path)
        vmd_type_str = ','.join(vmd_types)
    else:
        vmd_type_str = 'notvmd'
        
    # 通过文件位置获取作者，日期和歌曲名
    author, date, song_name, file_folder_middle_path = process_path(root, base_directory)
    
    # 插入作者表
    author_data = {
        'name' = author,
        'description' = 'unknown'
    }
    insert_into_table(cursor, 'authors', author_data)
    author_id = cursor.lastrowid  # 获取新插入记录的 file_id
    
    singer_names=['unknown']
    source = 'unknown'
    
    singer_ids = []
    song_id, singer_ids = add_song_and_singers_to_db(cursor, song_name, singer_names)

    # 准备写入或更新的数据
    file_data = {
        'file_name': file_name,
        'file_folder_path': file_folder_path,
        'file_folder_middle_path': file_folder_middle_path,
        'file_size': file_size,
        'file_hash': file_hash,
        'file_type': file_type,
        'vmd_type': vmd_type_str,
        'charName': charName,
        'boneKeyFrameNum': boneKeyFrameNum,
        'morphKeyFrameNum': morphKeyFrameNum,
        'cameraKeyFrameNum': cameraKeyFrameNum,
        'lightKeyFrameNum': lightKeyFrameNum,
        'selfShadowFrameNum': selfShadowFrameNum,
        'IKKeyFrameNum': IKKeyFrameNum,
        'source': source,
        'date': date
    }   

    if row:
        where_clause = {
            'file_name': file_name,
            'file_folder_path': file_folder_path
        } 
        update_table(cursor, 'files', file_data, where_clause)
        # Retrieve the file_id after update
        cursor.execute('SELECT file_id FROM files WHERE file_name = ? AND file_folder_path = ?', (file_name, file_folder_path))
        file_id = cursor.fetchone()[0]
    else:
        insert_into_table(cursor, 'files', file_data)
        file_id = cursor.lastrowid  # 获取新插入记录的 file_id
        # 在添加文件后立即链接到歌曲
    link_table(cursor, 'author_id', song_id, 'file_id', file_id)
    link_table(cursor, 'song_id', song_id, 'file_id', file_id)

def scan_directory_and_add_to_db(directory):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            add_file_to_db(cursor, file, root, update=True)
            
    conn.commit()
    conn.close()

# 扫描指定目录
scan_directory_and_add_to_db(base_directory)

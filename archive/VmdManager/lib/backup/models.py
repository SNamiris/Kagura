import os

from sqlalchemy import Column, Integer, String, ForeignKey, Text, UniqueConstraint, CheckConstraint
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship

from sqlalchemy import select, PrimaryKeyConstraint
from sqlalchemy.orm import aliased, joinedload
from sqlalchemy.exc import SQLAlchemyError
from PySide6.QtCore import QAbstractListModel, QModelIndex, Qt

import utils

Base = declarative_base()

# table name 是 class 的词汇小写用下划线分割
# 定义文件信息表
class File(Base):
    __tablename__ = 'file'
    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    folder_path = Column(String, nullable=False)
    type = Column(String, nullable=True)
    vmd_type = Column(String, nullable=True)
    size = Column(Integer, nullable=True)
    hash = Column(String, nullable=True)
    date = Column(String, nullable=True)
    source = Column(String, nullable=True)
    char_name = Column(String, nullable=True)
    bone_key_frame_num = Column(Integer, nullable=True)
    morph_key_frame_num = Column(Integer, nullable=True)
    camera_key_frame_num = Column(Integer, nullable=True)
    light_key_frame_num = Column(Integer, nullable=True)
    self_shadow_frame_num = Column(Integer, nullable=True)
    ik_key_frame_num = Column(Integer, nullable=True)

    __table_args__ = (UniqueConstraint('name', 'folder_path'),)
    
    song_files = relationship('SongFile', backref='file', cascade='all, delete-orphan')
    author_files = relationship('AuthorFile', backref='file', cascade='all, delete-orphan')
    file_tags = relationship("FileTag", backref='file', cascade='all, delete-orphan')

# 定义歌曲条目表
class Song(Base):
    __tablename__ = 'song'
    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    description = Column(Text)

    song_files = relationship("SongFile", backref='song', cascade='all, delete-orphan')

# 定义歌手条目表
class Singer(Base):
    __tablename__ = 'singer'
    id = Column(Integer, primary_key=True)
    name = Column(String, unique=True, nullable=False)
    
    song_singers = relationship("SongSinger", backref='singer', cascade='all, delete-orphan')

# 定义歌曲歌手关系表
class SongSinger(Base):
    __tablename__ = 'song_singer'
    song_id = Column(Integer, ForeignKey('song.id', ondelete="CASCADE"), primary_key=True)
    singer_id = Column(Integer, ForeignKey('singer.id', ondelete="CASCADE"), primary_key=True)

# 定义歌曲与文件关系表
class SongFile(Base):
    __tablename__ = 'song_file'
    song_id = Column(Integer, ForeignKey('song.id', ondelete="CASCADE"), primary_key=True)
    file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"), primary_key=True)

# 定义作者条目表
class Author(Base):
    __tablename__ = 'author'
    id = Column(Integer, primary_key=True)
    name = Column(String, unique=True, nullable=False)
    description = Column(Text)

    author_files = relationship('AuthorFile', backref='author', cascade='all, delete-orphan') 

# 定义作者文件关系表
class AuthorFile(Base):
    __tablename__ = 'author_file'
    author_id = Column(Integer, ForeignKey('author.id', ondelete="CASCADE"), primary_key=True)
    file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"), primary_key=True)

# 定义MMD演出方案表
class PerformancePlan(Base):
    __tablename__ = 'performance_plan'
    id = Column(Integer, primary_key=True)
    name = Column(String, unique=True, nullable=False)                             #名字
    dancer_number = Column(Integer, nullable=False)                   #舞者数量
    rating = Column(Integer, CheckConstraint('rating >= 0 AND rating <= 10')) #存储评分，1-10 的整数值, 0表示没有评分
    comments = Column(Text)                           #存储简评
    
    # Relationship definitions
    performance_plan_dancer = relationship('PerformancePlanDancer', backref='plan', cascade='all, delete-orphan')
    performance_plan_camera = relationship('PerformancePlanCamera', backref='plan', cascade='all, delete-orphan')
    performance_plan_music = relationship('PerformancePlanMusic', backref='plan', cascade='all, delete-orphan')
    performance_plan_song = relationship('PerformancePlanSong', backref='plan', cascade='all, delete-orphan')
    performance_plan_author = relationship('PerformancePlanAuthor', backref='plan', cascade='all, delete-orphan')
    performance_plan_tag = relationship('PerformancePlanTag', backref='plan', cascade='all, delete-orphan')
    performance_plan_playlist = relationship('PerformancePlanPlaylist', backref='plan', cascade='all, delete-orphan')

# 定义MMD演出方案与Dancer文件关系表
class PerformancePlanDancer(Base):
    __tablename__ = 'performance_plan_dancer'

    # 假设 plan_id 和两个不同的 file_id 共同构成复合主键
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"))
    motion_file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"))
    morph_file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"))
    sequence = Column(Integer, nullable=False)
    __table_args__ = (
        PrimaryKeyConstraint('plan_id', 'motion_file_id', 'morph_file_id', 'sequence'),
    )

    # 为每个 file_id 设置不同的 relationship，以便在 ORM 中可以区分并正确处理
    motion_file = relationship("File", foreign_keys=[motion_file_id])
    morph_file = relationship("File", foreign_keys=[morph_file_id])
    
# 定义MMD演出方案与镜头文件关系表
class PerformancePlanCamera(Base):
    __tablename__ = 'performance_plan_camera'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"))
    file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"))
    sequence = Column(Integer, nullable=False)
    __table_args__ = (
        PrimaryKeyConstraint('plan_id', 'file_id', 'sequence'),
    )
    
# 定义MMD演出方案与音乐文件关系表
class PerformancePlanMusic(Base):
    __tablename__ = 'performance_plan_music'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"))
    file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"))
    sequence = Column(Integer, nullable=False)
    __table_args__ = (
        PrimaryKeyConstraint('plan_id', 'file_id', 'sequence'),
    )

# 定义MMD演出方案与歌曲关系表
class PerformancePlanSong(Base):
    __tablename__ = 'performance_plan_song'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"), primary_key=True)
    song_id = Column(Integer, ForeignKey('song.id', ondelete="CASCADE"), primary_key=True)

# 定义MMD演出方案与作者关系表
class PerformancePlanAuthor(Base):
    __tablename__ = 'performance_plan_author'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"), primary_key=True)
    author_id = Column(Integer, ForeignKey('author.id', ondelete="CASCADE"), primary_key=True)

# 定义标签表
class Tag(Base):
    __tablename__ = 'tag'
    id = Column(Integer, primary_key=True)
    name = Column(String, unique=True, nullable=False)

    performance_plan_tags = relationship('PerformancePlanTag', backref='tag', cascade='all, delete-orphan')
    file_tags = relationship("FileTag", backref='tag', cascade='all, delete-orphan')

# 定义MMD演出方案与标签关系表
class PerformancePlanTag(Base):
    __tablename__ = 'performance_plan_tag'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"), primary_key=True)
    tag_id = Column(Integer, ForeignKey('tag.id', ondelete="CASCADE"), primary_key=True)

# 定义文件与标签关系表
class FileTag(Base):
    __tablename__ = 'file_tag'
    file_id = Column(Integer, ForeignKey('file.id', ondelete="CASCADE"), primary_key=True)
    tag_id = Column(Integer, ForeignKey('tag.id', ondelete="CASCADE"), primary_key=True)

# 定义演出列表表
class Playlist(Base):
    __tablename__ = 'playlist'
    id = Column(Integer, primary_key=True)
    name = Column(String, unique=True, nullable=False)

    performance_plan_playlist = relationship('PerformancePlanPlaylist', backref='playlist', cascade='all, delete-orphan')

# 定义MMD演出方案与演出列表关系表
class PerformancePlanPlaylist(Base):
    __tablename__ = 'performance_plan_playlist'
    plan_id = Column(Integer, ForeignKey('performance_plan.id', ondelete="CASCADE"))
    playlist_id = Column(Integer, ForeignKey('playlist.id', ondelete="CASCADE"))
    sequence = Column(Integer, nullable=False)
    __table_args__ = (
        PrimaryKeyConstraint('plan_id', 'playlist_id', 'sequence'),
    )

class BaseModel(QAbstractListModel):
    def __init__(self, session, fetch_size=20, parent=None):
        super().__init__(parent)
        self.session = session
        self.items = []
        self.fetch_size = fetch_size
        self.total_fetched = 0
        self.fetch_data()  # Initial load

    def fetch_data(self, search_text='', fetch_size=None ,filters=None, reset=False):
        if reset:
            self.total_fetched = 0
            self.items = []

        if not fetch_size:
            fetch_size = self.fetch_size
        
        query = self.session.query(self.entity)
        if search_text:
            query = query.filter(self.entity.name.like(f'%{search_text}%'))
        '''
        传入的字典形如
        filters = {
            'song_id': 1,
            'author_id': 2
        }
        '''
        query = self.build_query(query, filters)
        if query:
            query = query.offset(self.total_fetched).limit(fetch_size)
            new_items = query.all()

            if new_items:
                self.beginInsertRows(QModelIndex(), self.total_fetched, self.total_fetched + len(new_items) - 1)
                self.items.extend(new_items)
                self.total_fetched += len(new_items)
                self.endInsertRows()
            
        if reset:
            self.layoutChanged.emit()

        return self.items

    def fetch_all_data(self, search_text='', filters=None):
        query = self.session.query(self.entity)
        if search_text:
            query = query.filter(self.entity.name.like(f'%{search_text}%'))

        query = self.build_query(query, filters)
        if query:
            all_items = query.all()
            return all_items
        else:
            return []

    def build_query(self, query, filters):
        # Default implementation simply returns the query as is
        return query
    
    def search_by_id(self, id):
        item = self.session.query(self.entity).filter(self.entity.id == id).first()
        return item

    def search_by_name(self, name):
        items = self.session.query(self.entity).filter(self.entity.name == name).all()
        return items
    
    # 删除关联对应文件被删除的条目
    def cleanup(self, attr_name):
        items = self.session.query(self.entity).all()
        for item in items:
            value = getattr(item, attr_name, None)
            if not value:
                utils.log_and_print(f"Deleting orphaned parent: {item.name}")
                self.session.delete(item)

        self.session.commit()
    
    def deleteItem(self, index):
        item_id = self.items[index.row()].id
        # 删除数据库条目
        item = self.session.query(self.entity).filter_by(id=item_id).one()
        self.session.delete(item)
        self.session.commit()
        utils.log_and_print('item deleted')
        # 更新内部数据列表
        del self.items[index.row()]

    def saveChanges(self, data, relationship=None):
        try:
            self.session.begin_nested()  # Start a new transaction
            if 'id' in data and data['id'] != 0:
                # 更新逻辑
                item = self.session.query(self.entity).filter_by(id=data['id']).first()
                if not item:
                    raise ValueError(f"{self.entity.__name__} with id {data['id']} not found")
                for key, value in data.items():
                    setattr(item, key, value)
                self.process_relationship(item, relationship)
            else:
                # 创建逻辑
                data.pop('id', None)  # 移除 id 键，如果不存在则忽略
                item = self.entity(**data)
                self.session.add(item)
                self.session.flush()
                self.process_relationship(item, relationship)

            self.session.commit()
            utils.log_and_print(f"{self.entity.__name__} saved successfully.")
            return item
        except SQLAlchemyError as e:
            self.session.rollback()
            raise Exception(f"Failed to save changes: {str(e)}")
        
    def process_relationship(self, plan, relationship):
        # Default implementation simply returns None
        return None

    def rowCount(self, parent=QModelIndex()):
        return len(self.items)
    
    def clear_items(self):
        """清空列表并通知更新界面。"""
        self.items = []
        self.layoutChanged.emit()

    def data(self, index, role):
        if not index.isValid() or not (0 <= index.row() < len(self.items)):
            return None
        item = self.items[index.row()]
        return self.format_data(item, role)

    def format_data(self, item, role):
        if role == Qt.ItemDataRole.DisplayRole:
            return item.name
        elif role == Qt.ItemDataRole.UserRole:
            return item
        return None
    
    def removeRows(self, row, count, parent=QModelIndex()):
        self.beginRemoveRows(QModelIndex(), row, row + count - 1)
        self.items = self.items[:row] + self.items[row + count:]
        self.endRemoveRows()
        return True

class FileModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None, file_type=None, vmd_type=None):
        self.entity = File  # 指定此模型操作的实体类
        self.file_type = file_type
        self.vmd_type = vmd_type
        super().__init__(session, fetch_size, parent)

    def search_by_path(self, name, folder_path):
        item = self.session.query(File).filter(File.name == name, File.folder_path == folder_path).first()
        return item

    def set_tags(self, file_id, tag_ids):
        file = self.session.query(File).filter(File.id == file_id).one()
        file.file_tags.clear()
        for tag_id in tag_ids:
            # Corrected the syntax error in the query by fixing parentheses and removing the extra period.
            file_tag = FileTag(file_id=file_id, tag_id=tag_id)
            self.session.add(file_tag)
        self.session.flush()

    def fetch_song(self, file_id):
        # 查询返回完整的song对象
        results = self.session.query(Song).join(
            SongFile, SongFile.song_id == Song.id
        ).filter(
            SongFile.file_id == file_id
        ).all()

        return results
    
    def fetch_tags(self, file_id):
        # 查询返回完整的tag对象
        results = self.session.query(Tag).join(
            FileTag, FileTag.tag_id == Tag.id
        ).filter(
            FileTag.file_id == file_id
        ).all()

        return results
        

    def build_query(self, query, filters):
        if self.file_type:
            query = query.filter(File.type == self.file_type)
        if self.vmd_type:
            if self.vmd_type == 'dancer':
                query = query.filter(File.vmd_type.contains("motion"), File.vmd_type.contains("morph"))
            else:
                query = query.filter(File.vmd_type.contains(f"{self.vmd_type}"))

        if filters:
            field_to_table = {
                'song_id': SongFile,
                'author_id': AuthorFile,
                'tag_id': FileTag,
            }
            query = search_filter(query, self.entity, 'file_id', filters, field_to_table, self.clear_items)
        return query

    def format_data(self, file, role):
        if role == Qt.ItemDataRole.DisplayRole:
            file_path = os.path.join(file.folder_path, file.name)
            if file.type == 'vmd':
                if file.vmd_type == 'camera':
                    return f'key:{file.camera_key_frame_num}-{file_path}'
                elif file.vmd_type == 'motion':
                    return f'key:{file.bone_key_frame_num}-{file_path}'
                elif file.vmd_type == 'morph':
                    return f'key:{file.morph_key_frame_num}-{file_path}'
                else:
                    return f'motion:{file.bone_key_frame_num}-morph:{file.morph_key_frame_num}-{file_path}'
            return file_path
        elif role == Qt.ItemDataRole.UserRole:
            return file
        return None

class PerformancePlanModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = PerformancePlan  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

    def build_query(self, query, filters):
        if filters:
            field_to_table = {
                'song_id': PerformancePlanSong,
                'author_id': PerformancePlanAuthor,
                'tag_id': PerformancePlanTag,
                'playlist_id': PerformancePlanPlaylist
            }
            query = search_filter(query, self.entity, 'plan_id', filters, field_to_table, self.clear_items)
        return query

    def fetch_dancer_files(self, plan_id):
        # 使用别名对 motion_file 和 morph_file 进行连接
        MotionFile = aliased(File, name="MotionFile")
        MorphFile = aliased(File, name="MorphFile")

        # 修改查询以直接返回File对象，并按照sequence排序
        results = self.session.query(
            MotionFile,
            MorphFile
        ).select_from(PerformancePlanDancer
        ).join(
            MotionFile, PerformancePlanDancer.motion_file_id == MotionFile.id, isouter=True
        ).join(
            MorphFile, PerformancePlanDancer.morph_file_id == MorphFile.id, isouter=True
        ).filter(
            PerformancePlanDancer.plan_id == plan_id
        ).order_by(
            PerformancePlanDancer.sequence  # 添加排序条件
        ).all()

        # 初始化两个列表来分别存储不同类型的文件对象
        motion_files = []
        morph_files = []
        for motion_file, morph_file in results:
            if motion_file:
                motion_files.append(motion_file)
            if morph_file:
                morph_files.append(morph_file)

        return motion_files, morph_files

    def fetch_camera_files(self, plan_id):
        # 查询返回完整的File对象
        results = self.session.query(File).join(
            PerformancePlanCamera, PerformancePlanCamera.file_id == File.id
        ).filter(
            PerformancePlanCamera.plan_id == plan_id
        ).order_by(
            PerformancePlanCamera.sequence  # 添加排序条件
        ).all()

        # 返回的results列表包含的是File的完整对象
        return results

    def fetch_music_files(self, plan_id):
        # 查询返回完整的File对象
        results = self.session.query(File).join(
            PerformancePlanMusic, PerformancePlanMusic.file_id == File.id
        ).filter(
            PerformancePlanMusic.plan_id == plan_id
        ).order_by(
            PerformancePlanMusic.sequence  # 添加排序条件
        ).all()

        # 返回的results列表包含的是File的完整对象
        return results
      
    def fetch_tags(self, plan_id):
        # 查询返回完整的Tag对象
        results = self.session.query(Tag).join(
            PerformancePlanTag, PerformancePlanTag.tag_id == Tag.id
        ).filter(
            PerformancePlanTag.plan_id == plan_id
        ).all()

        # 返回的results列表包含的是Tag的完整对象
        return results
        
    def get_relationship_id(self, song_ids, author_ids, file_ids, tag_ids):
        # 传入 song_ids, author_ids集合
        for file_id in file_ids:
            file = self.session.query(File).filter_by(id=file_id).one()  # 查询文件实体
            for song_file in file.song_files:
                song_ids.add(song_file.song_id)  # 添加歌曲ID，自动去重
            for author_file in file.author_files:
                author_ids.add(author_file.author_id)  # 添加作者ID，自动去重
            for file_tag in file.file_tags:
                tag_ids.add(file_tag.tag_id)

        return song_ids, author_ids, tag_ids  # 将集合返回
    
    def process_relationship(self, plan, relationship): 
        plan_id = plan.id
        # Clear existing relationships if necessary
        plan.performance_plan_dancer.clear()
        plan.performance_plan_camera.clear()
        plan.performance_plan_music.clear()
        plan.performance_plan_song.clear()
        plan.performance_plan_author.clear()
        plan.performance_plan_tag.clear()
        # Assume the data contains lists of IDs for relationships
        dancer_files = relationship.get('dancer_file_ids', [])
        sequence = 0
        for dancer_file in dancer_files:
            sequence += 1
            self.session.add(PerformancePlanDancer(
                plan_id=plan_id, 
                motion_file_id=dancer_file['motion_file_id'], 
                morph_file_id=dancer_file['morph_file_id'],
                sequence=sequence
            ))
        sequence = 0
        for file_id in relationship.get('camera_file_ids', []):
            sequence += 1
            self.session.add(PerformancePlanCamera(plan_id=plan_id, file_id=file_id, sequence=sequence))
        sequence = 0
        for file_id in relationship.get('music_file_ids', []):
            sequence += 1
            self.session.add(PerformancePlanMusic(plan_id=plan_id, file_id=file_id, sequence=sequence))
        for song_id in relationship.get('song_ids', []):
            self.session.add(PerformancePlanSong(plan_id=plan_id, song_id=song_id))
        for author_id in relationship.get('author_ids', []):
            self.session.add(PerformancePlanAuthor(plan_id=plan_id, author_id=author_id))
        for tag_id in relationship.get('tag_ids', []):
            self.session.add(PerformancePlanTag(plan_id=plan_id, tag_id=tag_id))

    def is_exist(self, plan_exist, plan_data, motion_file_ids, morph_file_ids, camera_file_ids, music_file_ids):
        # Check if dancer numbers match
        if plan_exist.dancer_number != plan_data['dancer_number']:
            return False

        # Function to compare database file IDs with input file IDs
        def compare_file_ids(db_query, input_ids):
            # 将查询结果从列表包含的元组转换为单一的列表
            # 例如：[(1,), (2,)] -> [1, 2]
            extracted_ids = [item[0] for item in db_query]

            # 比较两个列表
            if extracted_ids !=  input_ids:
                return False

        # Check if camera file IDs match
        exist_camera_file_ids = self.session.query(
            PerformancePlanCamera.file_id
        ).filter(
            PerformancePlanCamera.plan_id == plan_exist.id
        ).order_by(
            PerformancePlanCamera.sequence  # 添加排序条件
        ).all()
        if not compare_file_ids(exist_camera_file_ids, camera_file_ids):
            return False

        # Check if music file IDs match
        exist_music_file_ids = self.session.query(
            PerformancePlanMusic.file_id
        ).filter(
            PerformancePlanMusic.plan_id == plan_exist.id
        ).order_by(
            PerformancePlanMusic.sequence  # 添加排序条件
        ).all()
        if not compare_file_ids(exist_music_file_ids, music_file_ids):
            return False

        # Check if dancer file IDs match (motion and morph combined)
        exist_dancer_file_ids = self.session.query(
            PerformancePlanDancer.motion_file_id,
            PerformancePlanDancer.morph_file_id
        ).filter(
            PerformancePlanDancer.plan_id == plan_exist.id
        ).order_by(
            PerformancePlanDancer.sequence  # 添加排序条件
        ).all()
        # 将查询结果转换为列表，列表中的每个元素是一个包含两个ID的元组
        db_dancer_list = [(item.motion_file_id, item.morph_file_id) for item in exist_dancer_file_ids]

        # 将输入的motion和morph ID组合成元组列表
        input_dancer_list = list(zip(motion_file_ids, morph_file_ids))

        # 比较两个列表
        return db_dancer_list == input_dancer_list

    # 删除关联对应文件被删除的条目
    def cleanup(self):
        items = self.session.query(self.entity).all()
        for item in items:
            if not item.performance_plan_dancer or not item.performance_plan_music:
                utils.log_and_print(f"Deleting orphaned parent: {item.name}")
                self.session.delete(item)

        self.session.commit()

    def format_data(self, plan, role):
        if role == Qt.ItemDataRole.DisplayRole:
            return f'{plan.name}[{plan.dancer_number}]'
        elif role == Qt.ItemDataRole.ToolTipRole:
            # 提供更多信息的提示，例如演出方案的详细描述
            return f"Plan ID: {plan.id}"
        elif role == Qt.ItemDataRole.UserRole:
            return plan
        return None
        
class SongModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = Song  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

    def build_query(self, query, filters):
        tag_id = None
        if filters:
            tag_id = filters.get("tag_id")
        if tag_id:
            query = query.join(SongFile, SongFile.song_id == Song.id).\
                join(FileTag, FileTag.file_id == SongFile.file_id).\
                filter(FileTag.tag_id == tag_id).distinct()
        return query

    def format_data(self, song, role):
        singers = self.session.query(Singer).\
                join(SongSinger, SongSinger.singer_id == Singer.id).\
                filter(SongSinger.song_id == song.id).\
                all()
        singer_text = ''
        for singer in singers:
            singer_text = singer_text + singer.name
        if role == Qt.ItemDataRole.DisplayRole:
            return f"id:{song.id} - {song.name} - " + singer_text
        elif role == Qt.ItemDataRole.ToolTipRole:
            return song.description
        elif role == Qt.ItemDataRole.UserRole:
            return song
        return None

class SingerModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = Singer  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

class AuthorModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = Author  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

    def format_data(self, author, role):
        if role == Qt.ItemDataRole.DisplayRole:
            return author.name
        elif role == Qt.ItemDataRole.ToolTipRole:
            return author.description
        elif role == Qt.ItemDataRole.UserRole:
            return author
        return None

class TagModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = Tag  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

class PlaylistModel(BaseModel):
    def __init__(self, session, fetch_size=20, parent=None):
        self.entity = Playlist  # 指定此模型操作的实体类
        super().__init__(session, fetch_size, parent)

    def fetch_plans(self, playlist_id):
        # 查询返回完整的File对象
        results = self.session.query(PerformancePlan).join(
            PerformancePlanPlaylist, PerformancePlanPlaylist.plan_id == PerformancePlan.id
        ).filter(
            PerformancePlanPlaylist.playlist_id == playlist_id
        ).order_by(
            PerformancePlanPlaylist.sequence  # 添加排序条件
        ).all()

        # 返回的results列表包含的是File的完整对象
        return results

    def process_relationship(self, playlist, relationship): 
        playlist_id = playlist.id
        # Clear existing relationships if necessary
        playlist.performance_plan_playlist.clear()
        # Assume the data contains lists of IDs for relationships
        sequence = 0
        for plan_id in relationship.get('plan_ids', []):
            sequence += 1
            self.session.add(PerformancePlanPlaylist(plan_id=plan_id, playlist_id=playlist_id, sequence=sequence))

def filters_to_tuples(filters, field_to_table):
    tuples_list = []
    for field, value in filters.items():
        table = field_to_table.get(field, None)
        if table:
            tuples_list.append((table, field, value))
    return tuples_list

def get_ids_query(table, filter_id, filter_column_name, target_column_name):
    filter_column = getattr(table, filter_column_name, None)
    target_column = getattr(table, target_column_name, None)
    if not filter_column or not target_column:
        raise AttributeError(f"One or both specified columns '{filter_column_name}' or '{target_column_name}' are not valid columns in {table.__tablename__}")
    # 生成查询
    return select(target_column).where(filter_column == filter_id)

def filter_to_ids_subquery(target_column_name, filters_tuples):
    filters = []
    ids_subquery = None
    # 处理额外的过滤条件
    for table, filter_column_name, filter_id in filters_tuples:
            filters.append(get_ids_query(table, filter_id, filter_column_name, target_column_name))

    # Combine filters
    if filters:
        ids_subquery = filters[0]
        for filter_query in filters[1:]:
            ids_subquery = ids_subquery.intersect(filter_query)
    
    return ids_subquery

def search_filter(query, table, target_column_name, filters, field_to_table, callback_on_empty=None):
    filters_tuples = filters_to_tuples(filters, field_to_table)
    '''
    传入的三元组列表
    [('songtable', 'song_id', 1), ('authortable', 'author_id', 2)]
    '''
    ids_subquery = filter_to_ids_subquery(target_column_name, filters_tuples)
    if ids_subquery is None:
        if callback_on_empty:
            callback_on_empty()
        return None
    else:
        query = query.filter(table.id.in_(ids_subquery))

    return query
import os
import sqlite3

# 确保目录存在
db_directory = '../database/'
if not os.path.exists(db_directory):
    os.makedirs(db_directory)

# 创建数据库连接
conn = sqlite3.connect(os.path.join(db_directory, 'MMDfiles_data.db'))
cursor = conn.cursor()

# 创建文件信息表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS files (
        file_id INTEGER PRIMARY KEY,
        file_name TEXT,
        file_folder_path TEXT,
        file_folder_middle_path TEXT, --除了作者时间歌名以外的路径
        file_size INTEGER,
        file_hash TEXT,     -- 文件哈希值
        file_type TEXT,     -- vmd, wav
        vmd_type TEXT,      -- motion, morph, camera
        charName TEXT, 
        boneKeyFrameNum INTEGER, 
        morphKeyFrameNum INTEGER, 
        cameraKeyFrameNum INTEGER, 
        lightKeyFrameNum INTEGER, 
        selfShadowFrameNum INTEGER, 
        IKKeyFrameNum INTEGER,
        source TEXT,        -- 假如有原视频地址
        date TEXT,
        UNIQUE(file_name, file_folder_path)
    )
''')

# 创建歌曲条目表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS songs (
        song_id INTEGER PRIMARY KEY,
        name TEXT,
        description TEXT  -- 详细描述，比如“由 Singer X 单独演唱”或“由 Singer X 和 Singer Y 合唱”
    )
''')

# 创建歌手条目表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS singers (
        singer_id INTEGER PRIMARY KEY,
        name TEXT UNIQUE         
    )
''')

# 创建歌曲歌手关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS song_singers (
        song_id INTEGER,
        singer_id INTEGER,
        PRIMARY KEY (song_id, singer_id),
        FOREIGN KEY(song_id) REFERENCES songs(song_id) ON DELETE CASCADE,
        FOREIGN KEY(singer_id) REFERENCES singers(singer_id) ON DELETE CASCADE
    )
''')

# 创建歌曲与文件关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS song_files (
        file_id INTEGER,
        song_id INTEGER,
        PRIMARY KEY (file_id, song_id),
        FOREIGN KEY(song_id) REFERENCES songs(song_id) ON DELETE CASCADE,
        FOREIGN KEY(file_id) REFERENCES files(file_id) ON DELETE CASCADE
    )
''')


# 创建作者条目表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS authors (
        author_id INTEGER PRIMARY KEY,
        name TEXT UNIQUE,
        description TEXT    --作者描述
    )
''')

# 创建作者文件关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS author_files (
        author_id INTEGER,
        file_id INTEGER,
        PRIMARY KEY (author_id, file_id),
        FOREIGN KEY(author_id) REFERENCES authors(author_id) ON DELETE CASCADE,
        FOREIGN KEY(file_id) REFERENCES files(file_id) ON DELETE CASCADE
    )
''')

# 创建MMD演出方案表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS performance_plans (
        setup_id INTEGER PRIMARY KEY AUTOINCREMENT,
        dancer_number INTEGER,       -- 舞者数量
        basefolder TEXT,             -- 基础文件夹路径
        dancer_motion TEXT,          -- Dancer的motion VMD
        dancer_morph TEXT,           -- Dancer的morph VMD（如果有）
        dancer_morph_id INTEGER,     -- 如果使用了不同morph，则给出对应motion的id，从0开始计数
        camera TEXT,                 -- Camera的VMD
        music TEXT,                  -- 音乐文件
        rating INTEGER CHECK(rating >= 1 AND rating <= 10),       -- 存储评分，1-10 的整数值
        comments TEXT               -- 存储简评
    )
''')

#创建MMD演出方案歌曲关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS performance_plan_songs (
        setup_id INTEGER,
        song_id INTEGER,
        PRIMARY KEY (setup_id, song_id),
        FOREIGN KEY(setup_id) REFERENCES performance_plans(setup_id) ON DELETE CASCADE,
        FOREIGN KEY(song_id) REFERENCES songs(song_id) ON DELETE CASCADE
    )
''')

#创建MMD演出方案作者关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS performance_plan_authors (
        setup_id INTEGER,
        author_id INTEGER,
        PRIMARY KEY (setup_id, author_id),
        FOREIGN KEY(setup_id) REFERENCES performance_plans(setup_id) ON DELETE CASCADE,
        FOREIGN KEY(author_id) REFERENCES authors(author_id) ON DELETE CASCADE
    )
''')

#创建标签表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS tags (
        tag_id INTEGER PRIMARY KEY,
        tag TEXT UNIQUE
    )
''')

#创建MMD演出方案标签关系表
cursor.execute('''
    CREATE TABLE IF NOT EXISTS performance_plan_tags (
        setup_id INTEGER,
        tag_id INTEGER,
        PRIMARY KEY (setup_id, tag_id),
        FOREIGN KEY(setup_id) REFERENCES performance_plans(setup_id) ON DELETE CASCADE,
        FOREIGN KEY(tag_id) REFERENCES tags(tag_id) ON DELETE CASCADE
    )
''')

conn.commit()
conn.close()

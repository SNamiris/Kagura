#简单地从作者给的文件夹直接生成playlist

import sqlite3
import configparser

# 读取配置文件
config = configparser.ConfigParser()
config.read('../config.ini')

# 获取路径信息
db_path = config['Paths']['db_path']

def generate_mmdd_playlist(cursor):
    # 查询歌曲及其关联的文件
    cursor.execute('''
        SELECT s.song_name, s.author, s.dancernumber, f.file_name, f.file_type, f.vmd_type, f.file_folder_path
        FROM songs s
        JOIN song_files sf ON s.song_id = sf.song_id
        JOIN files f ON sf.file_id = f.file_id
        ORDER BY s.song_name, f.file_type
    ''')
    results = cursor.fetchall()

    # 创建一个字典来存储每首歌曲的文件
    songs_dict = {}

    for song_name, author, dancernumber, file_name, file_type, vmd_type, file_folder_path in results:
        # 生成 basefolder 路径
        base_folder = f"{file_folder_path}"
        
        if song_name not in songs_dict:
            songs_dict[song_name] = {
                'basefolder': base_folder,
                'DancerNumber': dancernumber,
                'Dancers': [],
                'DancersMotion': [],
                'DancerMorph': [],
                'Cameras': [],
                'Music': []
            }

        if file_type == 'vmd':
            if 'motion' in vmd_type and 'morph' in vmd_type:
                songs_dict[song_name]['Dancers'].append(file_name)
            elif 'motion' in vmd_type and 'morph' not in vmd_type:
                songs_dict[song_name]['DancersMotion'].append(file_name)
            elif 'motion' not in vmd_type and 'morph' in vmd_type:
                songs_dict[song_name]['DancerMorph'].append(file_name)
            elif 'camera' in vmd_type:
                songs_dict[song_name]['Cameras'].append(file_name)
        elif file_type == 'wav':
            songs_dict[song_name]['Music'].append(file_name)

    # 生成输出
    playlist = ['; MMDDirector playlist\n\n']
    for song_name, song_data in songs_dict.items():
        playlist.append(f"[{song_name}]")
        playlist.append(f"basefolder={song_data['basefolder']}")
        playlist.append(f"DancerNumber={song_data['DancerNumber']}")
        
        for i, dancer in enumerate(song_data['Dancers']):
            playlist.append(f"Dancer{i}={dancer}")
        
        for i, camera in enumerate(song_data['Cameras']):
            playlist.append(f"Camera{i}={camera}")
        
        for i, music in enumerate(song_data['Music']):
            playlist.append(f"Music{i}={music}")
        
        playlist.append("")  # 添加空行分隔不同的歌曲
    
    return "\n".join(playlist)

# 使用示例
conn = sqlite3.connect(db_path)
cursor = conn.cursor()
playlist = generate_mmdd_playlist(cursor)
conn.close()

# 将playlist写入txt文件
with open('MMD_playlist.txt', 'w', encoding='utf-8') as file:
    file.write(playlist)

print("Playlist has been written to MMD_playlist.txt")



# 获取所有歌曲及其关联的文件信息
def generate_mmdd_playlist(cursor):
    cursor.execute('''
        SELECT s.song_name, f.author, f.file_name, f.file_type, f.vmd_type, f.file_folder_path
        FROM songs s
        JOIN song_files sf ON s.song_id = sf.song_id
        JOIN files f ON sf.file_id = f.file_id
        ORDER BY s.song_name, f.file_type
    ''')
    results = cursor.fetchall()

    songs_dict = {}

    for song_name, author, file_name, file_type, vmd_type, file_folder_path in results:
        base_folder = f"{file_folder_path}"
        
        # 如果author不存在，初始化一个空字典
        if song_name not in songs_dict:
            songs_dict[song_name] = {}

        # 如果song_name不存在，初始化一个结构
        if author not in songs_dict[song_name]:
            songs_dict[song_name][author] = {
                'basefolder': base_folder,
                'DancerNumber': 1,
                'Dancers': [],
                'DancersMotion': [],
                'DancerMorph': [],
                'Cameras': [],
                'Music': []
            }

        # 根据文件类型和vmd_type来分类
        if file_type == 'vmd':
            if 'motion' in vmd_type and 'morph' in vmd_type:
                songs_dict[author][song_name]['Dancers'].append(file_name)
            elif 'motion' in vmd_type and 'morph' not in vmd_type:
                songs_dict[author][song_name]['DancersMotion'].append(file_name)
            elif 'motion' not in vmd_type and 'morph' in vmd_type:
                songs_dict[author][song_name]['DancerMorph'].append(file_name)
            elif 'camera' in vmd_type:
                songs_dict[author][song_name]['Cameras'].append(file_name)
        elif file_type == 'wav':
            songs_dict[author][song_name]['Music'].append(file_name)

    return songs_dict

# 从数据库生成数据字典
songs_dict = generate_mmdd_playlist(cursor)
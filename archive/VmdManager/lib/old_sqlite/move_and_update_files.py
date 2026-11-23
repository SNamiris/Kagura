import os
import sqlite3
import shutil
import configparser

# 读取配置文件
config = configparser.ConfigParser()
config.read('../config.ini')

# 获取路径信息
db_path = config['Paths']['db_path']
base_directory = config['Paths']['base_directory']

def move_and_update_file(cursor, file_id, file_name, old_folder_path, author, date, song_name):
    # 获取相对于 base_directory 的相对路径（去除base_directory部分）
    relative_old_path = os.path.relpath(old_folder_path, base_directory)

    # 获取中间层级（从旧路径中去掉author和date部分）
    middle_path = os.path.relpath(relative_old_path, os.path.join(author, date))

    # 构建新的文件夹路径，保留中间层级
    new_folder_path = os.path.join(base_directory, author, date, middle_path, song_name)
    
    # 创建新文件夹（如果不存在）
    if not os.path.exists(new_folder_path):
        os.makedirs(new_folder_path)

    # 构建旧的文件路径和新的文件路径
    old_file_path = os.path.join(old_folder_path, file_name)
    new_file_path = os.path.join(new_folder_path, file_name)

    # 移动文件到新的路径
    if os.path.exists(old_file_path):
        shutil.move(old_file_path, new_file_path)
        print(f"Moved: {old_file_path} to {new_file_path}")
    else:
        print(f"File not found: {old_file_path}. Skipping.")

    # 更新数据库中的文件夹路径
    cursor.execute('UPDATE files SET file_folder_path = ? WHERE file_id = ?', (new_folder_path, file_id))

def move_and_update_all_files():
    # 连接到数据库
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    try:
        # 获取数据库中的所有文件信息
        cursor.execute('SELECT file_id, file_name, file_folder_path, author, date, song_name FROM files')
        files = cursor.fetchall()
        
        for file_id, file_name, old_folder_path, author, date, song_name in files:
            move_and_update_file(cursor, file_id, file_name, old_folder_path, author, date, song_name)
        # 提交更改
        conn.commit()
    finally:
        # 关闭数据库连接
        conn.close()
    print("Database updated successfully.")
    
def move_and_update_single_file(file_id=None, file_name=None):
    # 连接到数据库
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    try:
        # 如果提供了 file_id，优先使用 file_id，否则使用 file_name
        if file_id:
            cursor.execute('SELECT file_id, file_name, file_folder_path, author, date, song_name FROM files WHERE file_id = ?', (file_id,))
        elif file_name:
            cursor.execute('SELECT file_id, file_name, file_folder_path, author, date, song_name FROM files WHERE file_name = ?', (file_name,))
        else:
            print("Please provide either file_id or file_name.")
            return
            
        file = cursor.fetchone()

        if not file:
            print("File not found in the database.")
            return
            
        file_id, file_name, old_folder_path, author, date, song_name = file
        move_and_update_file(cursor, file_id, file_name, old_folder_path, author, date, song_name)
        
        # 提交更改
        conn.commit()
    finally:
        # 关闭数据库连接
        conn.close()
    print("Database updated successfully.")

# 执行移动和更新操作
move_and_update_all_files()

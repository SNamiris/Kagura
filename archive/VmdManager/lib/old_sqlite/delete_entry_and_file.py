import os
import sqlite3
import configparser

# 读取配置文件
config = configparser.ConfigParser()
config.read('../config.ini')

# 获取路径信息
db_path = config['Paths']['db_path']

def delete_entry_and_file(file_id=None, file_name=None, file_folder_path=None):
    # 连接到数据库
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    try:
        # 如果提供了 file_id，优先使用 file_id 获取文件信息
        if file_id:
            cursor.execute('SELECT file_id, file_name, file_folder_path FROM files WHERE file_id = ?', (file_id,))
        elif file_name and file_folder_path:
            cursor.execute('SELECT file_id, file_name, file_folder_path FROM files WHERE file_name = ? AND file_folder_path = ?', (file_name, file_folder_path))
        else:
            print("Please provide either file_id or both file_name and file_folder_path.")
            return
        
        file_record = cursor.fetchone()

        if not file_record:
            print("File record not found in the database.")
            return
        
        file_id, file_name, file_folder_path = file_record
        
        # 构建完整的文件路径
        file_path = os.path.join(file_folder_path, file_name)

        # 删除本地文件
        if os.path.exists(file_path):
            os.remove(file_path)
            print(f"Deleted file: {file_path}")
        else:
            print(f"File not found: {file_path}. Skipping deletion of file.")

        # 删除数据库中的条目
        cursor.execute('DELETE FROM files WHERE file_id = ?', (file_id,))
        print(f"Deleted database record for file_id: {file_id}")

        # 提交更改
        conn.commit()

    finally:
        # 关闭数据库连接
        conn.close()

# 使用示例
delete_entry_and_file(file_id=1)  # 通过 file_id 删除
# 或者
delete_entry_and_file(file_name="example.vmd", file_folder_path="G:/MMD/VmdManager/data/monotama_go/202401")  # 通过 file_name 和 file_folder_path 删除

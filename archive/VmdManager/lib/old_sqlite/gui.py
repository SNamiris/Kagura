import sqlite3
import tkinter as tk
from tkinter import ttk, messagebox

import configparser

# 读取配置文件
config = configparser.ConfigParser()
config.read('../config.ini')

# 获取路径信息
db_path = config['Paths']['db_path']

# 连接到数据库
conn = sqlite3.connect(db_path)
cursor = conn.cursor()

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
        
        if song_name not in songs_dict:
            songs_dict[song_name] = {
                'basefolder': base_folder,
                'DancerNumber': 1,
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

    return songs_dict

# 从数据库生成数据字典
songs_dict = generate_mmdd_playlist(cursor)


def create_gui():
    root = tk.Tk()
    root.title("MMD Playlist Manager")

    # 左侧列表显示所有歌曲
    frame_left = ttk.Frame(root)
    frame_left.pack(side="left", fill="y", padx=10, pady=10)
    
    listbox = tk.Listbox(frame_left)
    listbox.pack(fill="both", expand=True)
    
    # 填充列表框
    for song_name in songs_dict.keys():
        listbox.insert(tk.END, song_name)
    
    # 右侧显示选中的歌曲及文件
    frame_right = ttk.Frame(root)
    frame_right.pack(side="right", fill="both", expand=True, padx=10, pady=10)
    
    label_details = ttk.Label(frame_right, text="Details")
    label_details.pack(anchor="nw")

    # Motion, Morph, Camera, Music 列表框
    list_motion = tk.Listbox(frame_right, height=10)
    list_motion.pack(fill="x")
    
    list_morph = tk.Listbox(frame_right, height=10)
    list_morph.pack(fill="x")

    list_camera = tk.Listbox(frame_right, height=10)
    list_camera.pack(fill="x")
    
    list_music = tk.Listbox(frame_right, height=5)
    list_music.pack(fill="x")
    
    # 更新右侧详情框
    def update_details(event):
        selected_song = listbox.get(listbox.curselection())
        details = songs_dict[selected_song]
        
        # 清空列表框
        list_motion.delete(0, tk.END)
        list_morph.delete(0, tk.END)
        list_camera.delete(0, tk.END)
        list_music.delete(0, tk.END)
        
        # 更新内容
        for item in details['DancersMotion']:
            list_motion.insert(tk.END, item)
        
        for item in details['DancerMorph']:
            list_morph.insert(tk.END, item)
        
        for item in details['Cameras']:
            list_camera.insert(tk.END, item)
        
        for item in details['Music']:
            list_music.insert(tk.END, item)
    
    listbox.bind("<<ListboxSelect>>", update_details)

    # 保存按钮
    def save_performance_setup():
        selected_song = listbox.get(listbox.curselection())
        details = songs_dict[selected_song]
        
        # 假设你已经选择并填充了所有相关字段
        cursor.execute('''
            INSERT INTO performance_setups (song_id, author, dancer_number, basefolder, dancer_motion, dancer_morph, camera, music)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ''', (
            cursor.execute('SELECT song_id FROM songs WHERE song_name = ?', (selected_song,)).fetchone()[0],
            details['author'],
            details['DancerNumber'],
            details['basefolder'],
            ','.join(details['DancersMotion']),
            ','.join(details['DancerMorph']),
            ','.join(details['Cameras']),
            ','.join(details['Music'])
        ))
        conn.commit()
        messagebox.showinfo("保存成功", "演出方案已保存到数据库。")

    btn_save = ttk.Button(root, text="保存演出方案", command=save_performance_setup)
    btn_save.pack(side="bottom", pady=10)
    
    root.mainloop()

# 启动GUI
create_gui()

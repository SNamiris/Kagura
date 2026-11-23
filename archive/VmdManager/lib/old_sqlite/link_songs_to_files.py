def link_songs_to_files(cursor):
    # 获取所有的歌曲
    cursor.execute('SELECT song_id, song_name, author FROM songs')
    songs = cursor.fetchall()
    
    for song_id, song_name, author in songs:
        # 查找与该歌曲匹配的 VMD 和 WAV 文件
        cursor.execute('''
            SELECT file_id FROM files 
            WHERE song_name = ? AND author = ? AND (file_type = 'vmd' OR file_type = 'wav')
        ''', (song_name, author))
        files = cursor.fetchall()
        
        for (file_id,) in files:
            # 插入到 song_files 表中
            cursor.execute('''
                INSERT INTO song_files (song_id, file_id)
                VALUES (?, ?)
            ''', (song_id, file_id))

    # 提交更改
    conn.commit()
    print("Songs and files have been successfully linked.")

# 使用示例
link_songs_to_files(cursor)

import re
def extract_song_info(song_name, pattern_template):
    # 如果模板中包含歌手和歌名的占位符，尝试匹配
    if "{singer}" in pattern_template and "{song}" in pattern_template:
        pattern_string = pattern_template.replace("{singer}", "(?P<singer>.+?)").replace("{song}", "(?P<song>.+)")
        pattern = re.compile(pattern_string)
        match = pattern.search(song_name)
        if match:
            return {
                'singer': match.group('singer'),
                'song': match.group('song')
            }
    # 如果模板仅包含歌名占位符，假设整个字符串是歌曲名
    elif "{song}" in pattern_template:
        pattern_string = pattern_template.replace("{song}", "(?P<song>.+)")
        pattern = re.compile(pattern_string)
        match = pattern.search(song_name)
        if match:
            return {
                'singer': '未知歌手',
                'song': match.group('song')
            }

    # 如果没有匹配到任何模式，返回默认值
    return {
        'singer': '未知歌手',
        'song': song_name
    }
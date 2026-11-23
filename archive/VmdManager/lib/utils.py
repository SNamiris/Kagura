import os
import blake3
import logging

#计算文件hash
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

# 判断两个文件夹是否独立
def are_independent_paths(path1, path2):
    # 转换为绝对路径并规范化
    absolute_path1 = os.path.abspath(path1)
    absolute_path2 = os.path.abspath(path2)

    # 获取两个路径的驱动器部分
    drive1 = os.path.splitdrive(absolute_path1)[0]
    drive2 = os.path.splitdrive(absolute_path2)[0]

    # 检查是否在同一个驱动器
    if drive1 != drive2:
        # 如果不在同一个驱动器，不能比较路径，直接返回False或根据需要处理
        print("路径位于不同的驱动器上。")
        return True  # 或者其他适当的处理

    # 检查是否为相同路径或子路径
    common_path = os.path.commonpath([absolute_path1, absolute_path2])
    return common_path != absolute_path1 and common_path != absolute_path2

# 记录log并同时输出
def log_and_print(msg):
    print(msg)
    logging.info(msg)
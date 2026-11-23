import struct
import traceback
import chardet
from os import path
 
def check_vmd(filename, data):
    try:
        # check file
        if not path.isfile(filename):
            return ("File '%s' not found"%filename,)
        
        vmd_types = []
        encoding = "shift_jis"

        # open to read
        fp = open(filename, "rb")

        # header
        raw_data = fp.read(30).strip(b"\0")

        try:
            # 尝试使用shift_jis解码
            version = raw_data.decode("shift_jis")
        except UnicodeDecodeError:
            # 解码失败，使用chardet探测编码
            encoding = chardet.detect(raw_data)['encoding']
            try:
                # 使用探测到的编码重新尝试解码
                version = raw_data.decode(encoding)
            except UnicodeDecodeError:
                traceback.print_exc()
                return ("Vmd format error! " + str(e),)

        if version.startswith("Vocaloid Motion Data 0002"):
            name = fp.read(20)
        else:
            name = fp.read(10)
        # 将 Shift_JIS 字节串解码为 Unicode 字符串
        charName = name.strip(b"\x00").replace(b"\x00", b" ").decode(encoding, errors='ignore')

        # bone key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            boneKeyFrameNum = struct.unpack('I', numstr)[0]
        else:
            boneKeyFrameNum = 0
        if boneKeyFrameNum > 0:
            vmd_types.append('motion')
            fp.seek(boneKeyFrameNum * 111, 1)

        # morph key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            morphKeyFrameNum = struct.unpack('I', numstr)[0]
        else:
            morphKeyFrameNum = 0
        if morphKeyFrameNum > 0:
            vmd_types.append('morph')
            fp.seek(morphKeyFrameNum * 23, 1)
            
        # camera key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            cameraKeyFrameNum = struct.unpack('I', numstr)[0]
        else:
            cameraKeyFrameNum = 0
        if cameraKeyFrameNum > 0:
            vmd_types.append('camera')
            fp.seek(cameraKeyFrameNum * 61, 1)
                
        # light key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            lightKeyFrameNum = struct.unpack('I', numstr)[0]
        else:
            lightKeyFrameNum = 0
        if lightKeyFrameNum > 0:
            vmd_types.append('light')
            fp.seek(lightKeyFrameNum * 28, 1)
        
        #self shadow key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            selfShadowFrameNum = struct.unpack('I', numstr)[0]
        else:
            selfShadowFrameNum = 0
        if selfShadowFrameNum > 0:
            vmd_types.append('self shadow')
            fp.seek(selfShadowFrameNum * 9, 1)

        #IK key frame
        numstr = fp.read(4)
        if len(numstr) == 4:
            IKKeyFrameNum = struct.unpack('I', numstr)[0]
        else:
            IKKeyFrameNum = 0
        if IKKeyFrameNum > 0:
            vmd_types.append('IK')
        # close
        fp.close()
        
        data['char_name'] = charName
        data['bone_key_frame_num'] = boneKeyFrameNum
        data['morph_key_frame_num'] = morphKeyFrameNum
        data['camera_key_frame_num'] = cameraKeyFrameNum
        data['light_key_frame_num'] = lightKeyFrameNum
        data['self_shadow_frame_num'] = selfShadowFrameNum
        data['ik_key_frame_num'] = IKKeyFrameNum

        return vmd_types, data

    except Exception as e:
        traceback.print_exc()
        return ("Vmd format error! " + str(e),)

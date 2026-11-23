import sys
import os
import configparser
import shutil
import logging
from datetime import datetime
from itertools import zip_longest

from sqlalchemy import create_engine, event
from sqlalchemy.orm import sessionmaker

from PySide6.QtCore import QSettings, Qt, QEvent
from PySide6.QtWidgets import QApplication, QMainWindow, QSplitter, QMessageBox, QListWidgetItem, QMenu, QCompleter, QListView, QListWidget, QFileDialog
from PySide6.QtGui import QAction

import utils
import utils_file
from models import Base, FileModel, AuthorModel, SongModel, SingerModel, PerformancePlanModel, TagModel, PlaylistModel
from ui import Ui_mainWindow

global  FileDatabasePath, SQLDatabasePath, ExportPath, ExportType, SelectiveScanFolder, ScanAndDeleteDuplicateFolder, RearrangeFolder, RearrangeFormat, FolderFormat, SongNameFormat, flushed_but_not_committed

class MainWindow(QMainWindow):
    def __init__(self, session):
        super().__init__()
        self.session = session
        self.setup_ui()
        self.setup_menu()
        self.setup_models_and_views()
        
    def setup_ui(self):
        self.ui = Ui_mainWindow()  # 确保你有正确初始化 UI
        self.ui.setupUi(self)
        # 初始化窗口状态的恢复
        self.initUIState()
        
    def setup_menu(self):
        # Connect the action with a lambda that includes a callback to refresh models
        self.ui.actionScanFileDatabasePath.triggered.connect(self.scan_file_database_path)
        self.ui.actionBackupDatabase.triggered.connect(self.backup_database)
        self.ui.actionCleanDatabase.triggered.connect(self.clean_database)
        self.ui.actionCleanSongSingerAuthor.triggered.connect(self.clean_database_song_singer_author)

        self.ui.actionRestartWindow.triggered.connect(self.restart_window)
        self.ui.actionResetPPD.triggered.connect(self.reset_all_ppd)
        self.ui.actionResetPlaylist.triggered.connect(self.reset_all_playlist)
        
    def setup_models_and_views(self):
        # 设置PerformancePlan模型和视图
        self.performancePlanModel = self.setup_view_model(self.ui.pp_performance_plan, PerformancePlanModel, self.ui.pp_performance_plan_search)
        # 单击显示performance_plan
        self.ui.pp_performance_plan.clicked.connect(lambda index: self.performance_plan_click(index))
        # 设置保存performance_plan按钮
        self.ui.pp_save_plan.clicked.connect(self.save_performanceplan)
        # 设置删除performance_plan按钮
        self.ui.pp_delete_plan.clicked.connect(lambda : self.deleteModelItem(self.performancePlanModel, self.ui.pp_performance_plan))
        # 设置重置performance_plan_detail按钮
        self.ui.pp_new_plan.clicked.connect(self.reset_all_ppd)
        # 设置pp的右键菜单
        self.set_up_pp_menu(self.ui.pp_performance_plan, self.ui.pp_performance_plan_in_playlist)
        
        # 设置Song模型和视图，假设有相应的组件和模型
        self.songModel = self.setup_view_model(self.ui.pp_song, SongModel, self.ui.pp_song_search)
        # 根据双击歌曲筛选/不筛选文件和performanceplan
        self.ui.pp_song.doubleClicked.connect(lambda index: self.double_click_filter('song_id', index, self.ui.pp_filter))

        # 设置singer模型
        self.singerModel = SingerModel(self.session)

        # 设置Author模型和视图
        self.authorModel = self.setup_view_model(self.ui.pp_author, AuthorModel, self.ui.pp_author_search)
        # 根据双击作者筛选/不筛选文件和performanceplan
        self.ui.pp_author.doubleClicked.connect(lambda index: self.double_click_filter('author_id', index, self.ui.pp_filter))

        # 设置Tag模型和视图
        self.tagModel = self.setup_view_model(self.ui.pp_tag, TagModel, self.ui.pp_tag_search)
        # 根据双击tag筛选/不筛选performanceplan
        self.ui.pp_tag.doubleClicked.connect(lambda index: self.double_click_filter('tag_id', index, self.ui.pp_filter))
        # 设置ppd_tag_comboBox
        self.init_tag_comboBox(self.ui.pp_ppd_tag_comboBox)
        # 设置ppd_add_tag按钮
        self.ui.pp_ppd_add_tag.clicked.connect(lambda : self.add_tag_to_list(self.ui.pp_ppd_tag_comboBox, self.ui.pp_ppd_tags))
        # 设置删除Tag按钮
        self.ui.pp_delete_tag.clicked.connect(lambda : self.deleteModelItem(self.tagModel, self.ui.pp_tag))

        # 设置Playlist模型和视图
        self.playlistModel = self.setup_view_model(self.ui.pp_playlist, PlaylistModel, self.ui.pp_playlist_search)
        # 根据单击playlist项目显示相关的performanceplan
        self.ui.pp_playlist.clicked.connect(lambda index: self.playlist_click_filter(index, self.ui.pp_performance_plan_in_playlist))
        # 双击播放列表加载播放列表
        self.ui.pp_playlist.doubleClicked.connect(lambda index: self.playlist_double_click_filter(index, self.ui.pp_performance_plan_in_playlist))
        # 设置保存playlist按钮
        self.ui.pp_save_playlist.clicked.connect(self.save_playlist)
        # 设置重置playlist界面按钮
        self.ui.pp_new_playlist.clicked.connect(self.reset_all_playlist)
        # 设置删除playlist按钮
        self.ui.pp_delete_playlist.clicked.connect(lambda: self.deleteModelItem(self.playlistModel, self.ui.pp_playlist))
        # 设置导入外部playlist
        self.ui.pp_load_playlist.clicked.connect(self.load_playlist)
        # 设置导出Playlist
        self.ui.pp_export_playlist.clicked.connect(lambda: self.export_playlist(export_path=ExportPath, export_type=ExportType))
    
        # pp_performance_plan_in_playlist
        # 单击显示performance_plan信息
        self.ui.pp_performance_plan_in_playlist.clicked.connect(lambda index: self.performance_plan_click(index))
        # 设置单次导入所有pp_performance_plan中显示的plan（被筛选的plan）
        self.ui.pp_add_filtered_plan.clicked.connect(lambda: self.add_filtered_plan(self.ui.pp_filter, self.ui.pp_performance_plan_in_playlist))
        # 设置删除playlist中重复的plan条目
        self.ui.pp_remove_duplicate_entries.clicked.connect(lambda: self.remove_listWidget_duplicates(self.ui.pp_performance_plan_in_playlist))

        # 设置FileModel
        self.fileModel_dancer = self.setup_file_model(self.ui.pp_file_dancer, 'dancer', vmd_type='dancer')
        self.fileModel_dancer_motion = self.setup_file_model(self.ui.pp_file_dancer_motion, 'motion', vmd_type='motion')
        self.fileModel_dancer_morph = self.setup_file_model(self.ui.pp_file_dancer_morph, 'morph', vmd_type='morph')
        self.fileModel_camera = self.setup_file_model(self.ui.pp_file_camera, 'camera', vmd_type='camera')
        self.fileModel_music = self.setup_file_model(self.ui.pp_file_music, 'music', file_type='wav', additional_click_action=lambda index: self.set_plan_name(index, self.ui.pp_ppd_plan_name))
        # self.ui.pp_file_music.doubleClicked.connect(lambda index: self.double_click_filter('song_id', index, self.ui.pp_filter, same_song=True))

        # 设置ppd页面listwidget的右键菜单
        self.set_up_listWidget(self.ui.pp_ppd_dancer)
        self.set_up_listWidget(self.ui.pp_ppd_camera)
        self.set_up_listWidget(self.ui.pp_ppd_music)
        self.set_up_listWidget(self.ui.pp_ppd_tags)
        self.set_up_listWidget(self.ui.pp_performance_plan_in_playlist)

        # 设置fm界面
        self.ui.fm_folder_format.setText(FolderFormat)
        self.ui.fm_song_name_format.setText(SongNameFormat)

        self.ui.fm_selective_scan_folder.setText(SelectiveScanFolder)
        self.ui.fm_selective_scan.clicked.connect(self.selective_scan)
        self.ui.fm_commit_datebase.clicked.connect(self.commit_database)
        self.ui.fm_find_moved_files.clicked.connect(self.find_moved_files)

        self.ui.fm_scan_and_delete_duplicate_folder.setText(ScanAndDeleteDuplicateFolder)
        self.ui.fm_scan_and_delete_duplicate.clicked.connect(self.scan_folder_delete_duplicate)

        self.ui.fm_rearrange_files_folder.setText(RearrangeFolder)
        self.ui.fm_rearrange_format.setText(RearrangeFormat)
        self.ui.fm_rearrange_files.clicked.connect(self.rearrange_files)

    def commit_database(self):
        utils.log_and_print("commit")
        self.session.commit()

    def backup_database(self):
        source_path = SQLDatabasePath  # 设置要备份的文件路径
        directory, filename = os.path.split(source_path)  # 分离文件名和目录路径
        timestamp = datetime.now().strftime('%Y%m%d-%H%M%S')  # 获取当前时间戳
        backup_filename = f"{filename}-{timestamp}"  # 创建备份文件名
        backup_path = os.path.join(directory, backup_filename) 
        try:
            shutil.copy2(source_path, backup_path)  # 执行备份操作
            utils.log_and_print('File has been backed up successfully.')
        except Exception as e:
            QMessageBox.critical(self, 'Error', f'Failed to backup file: {str(e)}')

    def clean_database(self):
        self.backup_database()
        utils_file.handle_deleted_file(self.session, FileDatabasePath)
        self.authorModel.cleanup('author_files')
        self.songModel.cleanup('song_files')
        self.singerModel.cleanup('song_singers')
        self.performancePlanModel.cleanup('performance_plan_motion')
        self.refresh_all_models()
        utils.log_and_print("cleanup finished")

    def clean_database_song_singer_author(self):
        self.backup_database()
        self.authorModel.cleanup('author_files')
        self.songModel.cleanup('song_files')
        self.singerModel.cleanup('song_singers')
        self.refresh_all_models()
        utils.log_and_print("cleanup finished")

    def selective_scan(self):
        folder = self.select_folder(self.ui.fm_selective_scan_folder)
        if folder:
            self.scan_file_database_path(folder)

    def find_moved_files(self):
        folder = self.select_folder(self.ui.fm_selective_scan_folder)
        if folder:
            utils_file.find_moved_files(self.session, folder, FileDatabasePath)
            self.scan_file_database_path(folder, True)
        
    def select_folder(self, lineEdit=None):
        if lineEdit:
            initial_path = lineEdit.text()
        else:
            initial_path = FileDatabasePath
        options = QFileDialog.Options()
        folder = QFileDialog.getExistingDirectory(self, "选择文件夹", initial_path, options=options)
        return folder

    def scan_file_database_path(self, directory=None, update=None):
        self.backup_database()
        if not directory:
            directory = FileDatabasePath
        # 调用扫描方法
        folder_format = self.ui.fm_folder_format.text()
        song_name_format = self.ui.fm_song_name_format.text()
        if update == None:
            update = self.ui.fm_update.isChecked()
        if utils.are_independent_paths(directory, FileDatabasePath):
            utils.log_and_print("can't scan out of database")
            return
        utils_file.scan_directory_and_add_to_db(self.session, directory, FileDatabasePath, folder_format, song_name_format, update)
        # 刷新模型
        self.refresh_all_models()

    def scan_folder_delete_duplicate(self):
        scan_folder = self.select_folder(self.ui.fm_scan_and_delete_duplicate_folder)
        if not os.path.exists(scan_folder):
            utils.log_and_print("folder path error")
            return
        if not utils.are_independent_paths(scan_folder, FileDatabasePath):
            utils.log_and_print("can't scan database")
            return
        response = QMessageBox.question(self, 'Confirm Scan',
                                        f'Do you really want to scan {scan_folder}?',
                                        QMessageBox.Yes | QMessageBox.No,
                                        QMessageBox.No)
        if response == QMessageBox.Yes:
            utils_file.scan_folder_delete_duplicate(self.session, scan_folder)

    def rearrange_files(self):
        rearrange_folder = self.select_folder(self.ui.fm_rearrange_files_folder)
        rearrange_format = self.ui.fm_rearrange_format.text()
        origin_format = self.ui.fm_origin_format.text()
        if not os.path.exists(rearrange_folder):
            utils.log_and_print("folder path error")
        response = QMessageBox.question(self, 'Confirm Rearrange',
                                        f'Do you really want to Rearrange rearrange_folder?',
                                        QMessageBox.Yes | QMessageBox.No,
                                        QMessageBox.No)
        if response == QMessageBox.Yes:
            utils_file.rearrange_files(self.session, rearrange_folder, rearrange_format, origin_format, FileDatabasePath)


    # 设置除File模型以外的listView模型
    def setup_view_model(self, view, model_class, lineEdit_search=None):
        model = model_class(self.session)
        view.setModel(model)
        view.verticalScrollBar().valueChanged.connect(
            lambda: self.check_scroll(view, model)
        )
        if lineEdit_search:
            lineEdit_search.textChanged.connect(
                lambda text: self.update_search(model, text)
            )
        return model

    # 设置File模型
    def setup_file_model(self, view, model_type, vmd_type=None, file_type=None, additional_click_action=None):
        model = FileModel(self.session, vmd_type=vmd_type, file_type=file_type)
        view.setModel(model)
        view.doubleClicked.connect(lambda index: self.double_click_add(index, model_type))
        if additional_click_action:
            view.doubleClicked.connect(additional_click_action)
        view.verticalScrollBar().valueChanged.connect(
            lambda: self.check_scroll(view, model)
        )
        return model

    # 确认滚动条刷新listview        
    def check_scroll(self, view, model, search_text='', lineEdit=None):
        if lineEdit == None:
            lineEdit = self.ui.pp_filter
        filters = self.parse_fliter(lineEdit.text())
        if view.verticalScrollBar().value() == view.verticalScrollBar().maximum():
            model.fetch_data(search_text=search_text, filters=filters, reset=False)

    # 双击给PerformancePlanDetail增加信息
    @staticmethod
    def set_plan_name(index, lineEdit):
        # 读取listview的File模型文件对应歌名填入
        data = index.data(Qt.UserRole)
        lineEdit.setText(data.song_files[0].song.name)
         
    @staticmethod
    def change_spin_box_value(spin_box, value):
        current_value = spin_box.value()
        spin_box.setValue(current_value + value)

    # 双击file的listview，增加信息到PerformancePlanDetail
    def double_click_add(self, index, data_type):
        data = index.data(Qt.UserRole)
        data_dict = self.file_to_dict(data)
        tags = self.fileModel_dancer.fetch_tags(data.id)
        self.set_items_to_listWidegt('tag', tags, self.ui.pp_ppd_tags, self.tag_to_dict)
        if data_type == 'dancer':
            data_dict['vmd_type'] = 'motion'
            self.set_item_data_to_listWidget('file', data_dict, self.ui.pp_ppd_dancer)
            new_data_dict = data_dict.copy()
            new_data_dict['vmd_type'] = 'morph'
            self.set_item_data_to_listWidget('file', new_data_dict, self.ui.pp_ppd_dancer)
            self.change_spin_box_value(self.ui.pp_ppd_dancer_number, 1)
        elif data_type == 'motion' or data_type == 'morph':
            if data_type == 'motion':
                self.change_spin_box_value(self.ui.pp_ppd_dancer_number, 1)
            data_dict['vmd_type'] = data_type
            self.set_item_data_to_listWidget('file', data_dict, self.ui.pp_ppd_dancer)
        elif data_type == 'camera':
            self.set_item_data_to_listWidget('file', data_dict, self.ui.pp_ppd_camera)
        elif data_type == 'music':
            self.set_item_data_to_listWidget('file', data_dict, self.ui.pp_ppd_music)

    @staticmethod
    def set_item_data_to_listWidget(type, data, listWidget):
        count = listWidget.count()
        itme_text = str(count+1) + ': '
        if type == 'file':
            file_path = os.path.join(data.get('folder_path'), data.get('name'))
            if data.get('vmd_type') == 'motion':
                item_text = itme_text + f"{data['vmd_type']}-key:{data['bone_key_frame_num']}-{file_path}"
            elif data.get('vmd_type') == 'morph':
                item_text = itme_text + f"{data['vmd_type']}-key:{data['morph_key_frame_num']}-{file_path}"
            elif data.get('vmd_type') == 'camera':
                item_text = itme_text + f"{data['vmd_type']}-key:{data['camera_key_frame_num']}-{file_path}"
            elif data.get('vmd_type') == 'notvmd': 
                item_text = itme_text + f"music-{file_path}"
            else:
                item_text = itme_text + file_path
        elif type == 'plan':
            item_text = itme_text + f"{data['name']}[{data['dancer_number']}]"
        elif type == 'tag':
            if any(listWidget.item(i).text() == data['name'] for i in range(listWidget.count())):
                print('tag existed')
                return 
            item_text = f"{data['name']}"
        item = QListWidgetItem(item_text)
        item.setData(Qt.UserRole, data)
        listWidget.addItem(item)

    # 单击显示performance_plan处理
    def performance_plan_click(self, index):
        self.reset_all_ppd_listWSidget()
        plan = self.get_item_data(index, self.plan_to_dict)
        self.ui.pp_ppd_plan_id.setText(str(plan['id']))
        self.ui.pp_ppd_plan_name.setText(plan['name'])
        self.ui.pp_ppd_dancer_number.setValue(plan['dancer_number'])
        self.ui.pp_ppd_rating.setValue(plan['rating'])
        self.ui.pp_ppd_comments.setText(plan['comments'])
        
        motion_files = self.performancePlanModel.fetch_motion_files(plan['id'])
        morph_files = self.performancePlanModel.fetch_morph_files(plan['id'])
        camera_files = self.performancePlanModel.fetch_camera_files(plan['id'])
        music_files = self.performancePlanModel.fetch_music_files(plan['id'])
        tags = self.performancePlanModel.fetch_tags(plan['id'])

        motion_files_info = [self.file_to_dict(file, 'motion') for file in motion_files]
        morph_files_info = [self.file_to_dict(file, 'morph') for file in morph_files]
        for motion_file_info, morph_file_info in zip_longest(motion_files_info, morph_files_info):
            if motion_file_info:
                self.set_item_data_to_listWidget('file', motion_file_info, self.ui.pp_ppd_dancer)
            if morph_file_info:
                self.set_item_data_to_listWidget('file', morph_file_info, self.ui.pp_ppd_dancer)

        self.set_items_to_listWidegt('file', camera_files, self.ui.pp_ppd_camera, self.file_to_dict)
        self.set_items_to_listWidegt('file', music_files, self.ui.pp_ppd_music, self.file_to_dict)
        self.set_items_to_listWidegt('tag', tags, self.ui.pp_ppd_tags, self.tag_to_dict)

    def get_item_data(self, index, process_function):
        item = index.data(Qt.UserRole)
        if isinstance(item, dict):
            # 是字典
            return item
        else:
            # 是orm对象
            item_dict = process_function(item)
            return item_dict

    # 根据单击playlist项目显示相关的performanceplan
    def playlist_click_filter(self, index, listWidget):
        playlist = index.data(Qt.UserRole)
        listWidget.clear()
        self.ui.pp_playlist_name.setText(playlist.name)
        plans = self.playlistModel.fetch_plans(playlist.id)
        self.set_items_to_listWidegt('plan', plans, listWidget, self.plan_to_dict)
        return playlist

    # 双击playlist项目加载该playlist
    def playlist_double_click_filter(self, index, listWidget):
        playlist = self.playlist_click_filter(index, listWidget)
        self.ui.pp_playlist_id.setText(str(playlist.id))

    # 将已过滤的performanceplan加入performance plan in playlist
    def add_filtered_plan(self, lineEdit, listWidget):
        filters = self.parse_fliter(lineEdit.text()) 
        plans = self.performancePlanModel.fetch_all_data(search_text=self.ui.pp_performance_plan_search.text(), filters=filters)
        self.set_items_to_listWidegt('plan', plans, listWidget, self.plan_to_dict)

    def set_up_pp_menu(self, listView, lineWidget):
        listView.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
        listView.customContextMenuRequested.connect(lambda position: self.pp_show_context_menu(position, listView, lineWidget))      

    def pp_show_context_menu(self, position, listView, listWidget):
        # 创建右键菜单
        menu = QMenu()
        add_to_playlis_action = QAction('AddToPlaylis', self)
        # 触发增加至playlist操作
        add_to_playlis_action.triggered.connect(lambda index: self.add_plan_to_playlist(index, listView, listWidget))
        menu.addAction(add_to_playlis_action)

        # 在鼠标右键点击的位置显示菜单
        menu.exec(listView.viewport().mapToGlobal(position))

    def add_plan_to_playlist(self, index, listView, listWidget):
        selected_indexes = listView.selectedIndexes()
        item_list = []
        for index in selected_indexes:
            item_list.append(index.data(Qt.UserRole))
        self.set_items_to_listWidegt('plan', item_list, listWidget, self.plan_to_dict)

    def set_items_to_listWidegt(self, type, items, listWidget, process_function):
        items_info = [process_function(item) for item in items]
        for item_info in items_info: 
            self.set_item_data_to_listWidget(type, item_info, listWidget)

    @staticmethod
    def file_to_dict(item, vmd_type=None):
        item_dict = {
            'id': item.id,
            'name': item.name,
            'folder_path': item.folder_path,
            'vmd_type': item.vmd_type,
            'bone_key_frame_num': item.bone_key_frame_num,
            'morph_key_frame_num': item.morph_key_frame_num,
            'camera_key_frame_num': item.camera_key_frame_num,
        }
        if vmd_type:
            if vmd_type == 'motion':
                item_dict['vmd_type'] = 'motion'
            elif vmd_type == 'morph':
                item_dict['vmd_type'] = 'morph'
        return item_dict

    @staticmethod
    def plan_to_dict(item):
        item_dict = {
            'id': item.id,
            'name': item.name,
            'dancer_number': item.dancer_number,
            'rating': item.rating,
            'comments': item.comments
        } 
        return item_dict

    @staticmethod
    def tag_to_dict(item):
        item_dict = {
            'id': item.id,
            'name': item.name
        }
        return item_dict

    # 设置ppd页面listwidget的右键菜单
    def set_up_listWidget(self, listWidget):
        # listWidget.currentItemChanged.connect(lambda: self.update_list_items_with_indices(listWidget))
        listWidget.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
        listWidget.customContextMenuRequested.connect(lambda position: self.ppd_show_context_menu(position, listWidget))           

    #PerformancePlanDetail界面与performance_plan_in_playlist条目右键删除
    def ppd_show_context_menu(self, position, listWidget):
        # 创建右键菜单
        menu = QMenu()
        delete_action = QAction('Delete', self)
        # 当触发删除操作时，直接调用删除函数，并传递当前的 listWidget
        delete_action.triggered.connect(lambda: self.delete_selected_items(listWidget))
        menu.addAction(delete_action)

        # 在鼠标右键点击的位置显示菜单
        menu.exec(listWidget.viewport().mapToGlobal(position))

    @staticmethod
    def update_list_items_with_indices(listWidget):
        # 更新所有项，添加序号
        for index in range(listWidget.count()):
            current_item = listWidget.item(index)
            original_text = current_item.text().split(":")[-1]  # 假设原始文本在" - "后
            current_item.setText(f"{index + 1}:{original_text}")

    def delete_selected_items(self, listWidget):
        # 删除选中的条目
        for item in listWidget.selectedItems():
            if listWidget == self.ui.pp_ppd_dancer:
                data = item.data(Qt.ItemDataRole.UserRole)
                if data['vmd_type'] == 'motion':
                    self.change_spin_box_value(self.ui.pp_ppd_dancer_number, -1)
            row = listWidget.row(item)
            listWidget.takeItem(row)

    # 初始化comboBox
    def init_tag_comboBox(self, comboBox):
        comboBox.clear()  # 清空 comboBox
        tags = self.tagModel.fetch_data()
        if tags:
            for tag in tags:
                tag_dict = self.tag_to_dict(tag)
                comboBox.addItem(tag.name, tag_dict)

        # 设置或更新自动补全
        self.tagcompleter = QCompleter([tag.name for tag in tags])
        self.tagcompleter.setCaseSensitivity(Qt.CaseInsensitive)
        self.tagcompleter.setCompletionMode(QCompleter.PopupCompletion)
        comboBox.setCompleter(self.tagcompleter)

    def add_tag_to_list(self, comboBox, listWidget):
        current_tag_text = comboBox.currentText()
        index = comboBox.findText(current_tag_text)

        item = QListWidgetItem(current_tag_text)
        if index != -1:
            # 如果找到已有标签，直接使用已有数据
            item.setData(Qt.UserRole, comboBox.itemData(index))
        else:
            # 添加新标签
            utils.log_and_print("add new tag")
            tag = self.add_new_tag(self.tagModel, current_tag_text)
            # 更新 comboBox 和 completer，只添加新标签
            comboBox.addItem(tag['name'], tag)
            self.tagcompleter.model().setStringList([comboBox.itemText(i) for i in range(comboBox.count())])

        listWidget.addItem(item)

    def add_new_tag(self, model, name):
        raw_tag_data = {
            'id': 0,
            'name': name
        }
        tag = model.saveChanges(raw_tag_data)
        self.update_search(model, search_text=self.ui.pp_tag_search.text())
        tag_data = {
            'id': tag.id,
            'name': tag.name
        }
        return tag_data

    # 保存数据库条目和listview条目
    def save_performanceplan(self):
        plan_data = {
            'id': int(self.ui.pp_ppd_plan_id.text()),
            'name': self.ui.pp_ppd_plan_name.text(),
            'dancer_number': self.ui.pp_ppd_dancer_number.value(),
            'rating': self.ui.pp_ppd_rating.value(),
            'comments': self.ui.pp_ppd_comments.toPlainText() 
        }
        motion_file_ids, morph_file_ids = self.get_dancer_info(self.ui.pp_ppd_dancer)
        camera_file_ids = self.get_item_ids(self.ui.pp_ppd_camera)
        music_file_ids = self.get_item_ids(self.ui.pp_ppd_music)
        tag_ids = self.get_item_ids(self.ui.pp_ppd_tags)
        file_ids = set(motion_file_ids + morph_file_ids + camera_file_ids + music_file_ids)
        # 将plan的tag全部赋值到文件里
        for file_id in file_ids:
            self.fileModel_dancer.set_tags(file_id, tag_ids)
        relationship = self.generate_relationship(motion_file_ids, morph_file_ids, camera_file_ids, music_file_ids, tag_ids)
        utils.log_and_print(plan_data)
        utils.log_and_print(relationship)
        self.performancePlanModel.saveChanges(plan_data, relationship)
        self.update_search(self.performancePlanModel, self.ui.pp_performance_plan_search.text(), self.parse_fliter(self.ui.pp_filter.text()))
        self.reset_all_ppd()

    @staticmethod
    def get_dancer_info(listWidget):
        motion_file_ids = []
        morph_file_ids = []
        for i in range(listWidget.count()):
            item = listWidget.item(i)
            file_info = item.data(Qt.UserRole)  # 假设您之前以这种方式存储了文件信息
            # 假设 file_info 是一个字典，包含了 'id', 'vmd_type' 等键
            if file_info['vmd_type'] == 'motion':
                motion_file_ids.append(file_info['id'])
            elif file_info['vmd_type'] == 'morph':
                morph_file_ids.append(file_info['id'])

        return motion_file_ids, morph_file_ids
    
    @staticmethod
    def get_item_ids(list):
        id_list = []
        if isinstance(list, QListWidget):
            for i in range(list.count()):
                item = list.item(i)
                file_info = item.data(Qt.UserRole) 
                id_list.append(file_info['id'])
        elif isinstance(list, QListView):
            model = list.model()
            if model:
                for row in range(model.rowCount()):
                    index = model.index(row, 0)  # 假设ID存储在第一列
                    file_info = model.data(index, Qt.ItemDataRole.UserRole)
                    if file_info and 'id' in file_info:
                        id_list.append(file_info['id'])
        return id_list

    def generate_relationship(self, motion_file_ids, morph_file_ids, camera_file_ids, music_file_ids, tag_ids):
        motion_file_ids = [] if motion_file_ids is None else motion_file_ids
        morph_file_ids = [] if morph_file_ids is None else morph_file_ids
        camera_file_ids = [] if camera_file_ids is None else camera_file_ids
        music_file_ids = [] if music_file_ids is None else music_file_ids
        # 全是列表
        file_ids = set(motion_file_ids + morph_file_ids + camera_file_ids + music_file_ids)
        song_ids, author_ids, file_tags = self.fileModel_dancer.get_file_relationship(file_ids)
        tag_ids = set(tag_ids)
        tag_ids = tag_ids.union(file_tags)
        relationship = {
            'motion_file_ids': motion_file_ids,
            'morph_file_ids': morph_file_ids,
            'camera_file_ids': camera_file_ids,
            'music_file_ids': music_file_ids,
            'tag_ids': tag_ids,
            'song_ids': song_ids,
            'author_ids': author_ids
        } 
        return relationship

    def save_playlist(self):
        plan_ids = self.get_item_ids(self.ui.pp_performance_plan_in_playlist)
        playlist_data = {
            'id': int(self.ui.pp_playlist_id.text()),
            'name': self.ui.pp_playlist_name.text(),
            'count':len(plan_ids)
        }
        relationship = {
            'plan_ids': plan_ids
        }
        self.playlistModel.saveChanges(playlist_data,relationship)
        self.reset_all_playlist()

    # 删除数据库条目和listview条目
    def deleteModelItem(self, model, listView):
        index = listView.currentIndex()
        if not index.isValid():
            QMessageBox.information(self, 'Selection', 'Please select an item to delete.')
            return

        response = QMessageBox.question(self, 'Confirm Deletion',
                                        'Do you really want to delete the selected item?',
                                        QMessageBox.Yes | QMessageBox.No,
                                        QMessageBox.No)
        if response == QMessageBox.Yes:
            if model:
                # 删除数据库条目
                model.deleteItem(index)

    # 根据双击筛选/不筛选文件
    def double_click_filter(self, filter_type, index, lineEdit, same_song = False):
        data = index.data(Qt.UserRole)  # Assuming UserRole returns the song_id
        # 如果开启选择某个文件，只显示与其相同song文件的功能
        if same_song:
            data = self.fileModel_dancer.fetch_songs(data.id)[0]
        new_filter = f'{data.id}={data.name}'
        self.set_filter(filter_type, new_filter, lineEdit)
        filters = self.parse_fliter(lineEdit.text()) 
        self.update_search(self.fileModel_dancer, filters=filters)
        self.update_search(self.fileModel_dancer_motion, filters=filters)
        self.update_search(self.fileModel_dancer_morph, filters=filters)
        self.update_search(self.fileModel_camera, filters=filters)
        self.update_search(self.fileModel_music, filters=filters)
        self.update_search(self.performancePlanModel, search_text=self.ui.pp_performance_plan_search.text(), filters=filters)
        if filter_type == 'tag_id':
            self.update_search(self.songModel, search_text=self.ui.pp_song_search.text(), fetch_size = 9999, filters=filters)

    # 读取外部播放文件
    def load_playlist(self):
        self.backup_database()
        # Open file dialog to select a file
        options = QFileDialog.Options()
        filename, _ = QFileDialog.getOpenFileName(self, "Select File", FileDatabasePath, "All Files (*);;Text Files (*.txt)", options=options)
        if filename:
            self.add_loaded_playlist(filename)

    def add_loaded_playlist(self, filename):
        playlist = self.parse_playlist_file(filename)
        self.process_playlist_file_dict(playlist, filename)

    @staticmethod
    def parse_playlist_file(filename):
        expected_header = '; MMDDirector playlist'
        file_directory = os.path.dirname(filename)

        def isabs(line, basefolder):
            path_part = line.split('=')[1].strip() # 获取路径部分并去除可能的前后空格
            # 检查是否是绝对路径
            if os.path.isabs(path_part):
                return path_part
            else:
                # 如果是相对路径，则与文件目录拼接
                return os.path.join(basefolder, path_part)

        try:
            with open(filename, 'r', encoding='utf-8') as file:
                # 读取第一行来检查文件头部
                header = file.readline().strip()
                if header != expected_header:
                    utils.log_and_print(f"Unexpected file format. Expected header '{expected_header}', but found '{header}'")
                    return None
                
                # 读取剩余的文件内容
                lines = file.readlines()

            playlist = {}
            current_section = None

            for line in lines:
                line = line.strip()
                if line.startswith('[') and line.endswith(']'):
                    current_section = line[1:-1]
                    playlist[current_section] = {
                        'basefolder': file_directory,
                        'DancerNumber': 0,
                        'DancersMotion': [],
                        'DancersMorph': [],
                        'Cameras': [],
                        'Music': []
                    }
                elif line.startswith('basefolder='):
                    playlist[current_section]['basefolder'] = isabs(line, playlist[current_section]['basefolder'])
                elif line.startswith('DancerNumber='):
                    playlist[current_section]['DancerNumber'] = int(line.split('=')[1])
                elif line.startswith('Dancer'):
                    vmd_type = line.split('=')[0]
                    if 'Motion' in vmd_type:
                        playlist[current_section]['DancersMotion'].append(isabs(line, playlist[current_section]['basefolder']))
                    elif 'Morph' in vmd_type:
                        playlist[current_section]['DancersMorph'].append(isabs(line, playlist[current_section]['basefolder']))
                    else:
                        playlist[current_section]['DancersMotion'].append(isabs(line, playlist[current_section]['basefolder']))
                        playlist[current_section]['DancersMorph'].append(isabs(line, playlist[current_section]['basefolder']))
                elif line.startswith('Camera'):
                    playlist[current_section]['Cameras'].append(isabs(line, playlist[current_section]['basefolder']))
                elif line.startswith('Music'):
                    playlist[current_section]['Music'].append(isabs(line, playlist[current_section]['basefolder']))

        except FileNotFoundError:
            utils.log_and_print(f"File not found: {filename}")
            return None
        except ValueError as e:
            utils.log_and_print(f'Value Error  {str(e)}')
            return None
        except Exception as e:
            utils.log_and_print(f"Error processing file {filename}: {str(e)}")
            return None

        return playlist

    def process_playlist_file_dict(self, playlist, filename):
        plan_ids = []
        for plan_name, plan in playlist.items():
            plan_data = {
                'id': 0,
                'name': plan_name,
                'dancer_number': plan['DancerNumber'],
                'rating': 0,
                'comments': '无评论' 
            }
            motion_file_paths, motion_file_ids = self.process_playlist_file_paths(plan['DancersMotion'], plan_name, 'DancersMotion')
            morph_file_paths, morph_file_ids = self.process_playlist_file_paths(plan['DancersMorph'], plan_name, 'DancersMorph')
            if motion_file_paths is None:
                continue
            if len(motion_file_ids) != plan['DancerNumber']:
                utils.log_and_print("motion file number is not equal to morph file number or dancer_number")
                continue
            if len(motion_file_ids) != len(morph_file_ids):
                utils.log_and_print("motion file number is not equal to morph file number or dancer_number")
            camera_file_paths, camera_file_ids = self.process_playlist_file_paths(plan['Cameras'], plan_name, 'Cameras')
            music_file_paths, music_file_ids = self.process_playlist_file_paths(plan['Music'], plan_name, 'Music')
            plan_exist = self.performancePlanModel.search_by_name(name=plan_name)
            if plan_exist != []:
                plan_exist = plan_exist[0]
                if self.performancePlanModel.is_exist(plan_exist, plan_data, motion_file_ids, morph_file_ids, camera_file_ids, music_file_ids):
                    utils.log_and_print(f"Performance Plan {plan_name} already exists, skipping")
                    continue
                else:
                    i = 1
                    while self.performancePlanModel.search_by_name(name=f'{plan_name}_{i}') != []:
                        i = i + 1
                    plan_data['name'] = plan_name + '_' + str(i)
            relationship = self.generate_relationship(motion_file_ids, morph_file_ids, camera_file_ids, music_file_ids, set())
            plan_ids.append(self.performancePlanModel.saveChanges(plan_data, relationship).id)
        playlist_data = {
            'name': os.path.splitext(os.path.basename(filename)),
            'count':len(plan_ids)
        }
        relationship = {
            'plan_ids': plan_ids
        }
        self.playlistModel.saveChanges(playlist_data, relationship)
        self.refresh_all_models()

    def process_playlist_file_paths(self, paths, plan_name, path_type):
        processed_paths = []
        processed_ids = []
        for path in paths:
            normalized_path = os.path.normpath(path)
            if not os.path.exists(normalized_path):
                utils.log_and_print(f"{plan_name} {path_type} file: {normalized_path} doesn't exist.")
                return None, None
            normalized_path = os.path.relpath(normalized_path, start=FileDatabasePath)
            processed_paths.append(normalized_path) 
            processed_file = self.fileModel_dancer.search_by_path(os.path.basename(normalized_path), os.path.dirname(normalized_path))
            if processed_file:
                processed_id = processed_file.id
                processed_ids.append(processed_id)
            else:
                utils.log_and_print(f"{plan_name} {path_type} file: {normalized_path} doesn't exist in FileDataBase.")
                return None, None
        return processed_paths, processed_ids

    def export_playlist(self, export_path=None, export_type='rel'):
        if not export_path:
            export_path = FileDatabasePath    

        plans = self.get_list_data(self.ui.pp_performance_plan_in_playlist)
        playlist_name = self.ui.pp_playlist_name.text()
        if playlist_name == '':
            utils.log_and_print("no playlist")
            return
        playlist = ['; MMDDirector playlist\n']
        for plan in plans:
            playlist.append(f"[{plan['name']}]")
            # 删掉basefolder, 将路径全部写在文件路径中
            playlist.append(f"DancerNumber={plan['dancer_number']}")

            motion_files = self.performancePlanModel.fetch_motion_files(plan['id'])
            morph_files = self.performancePlanModel.fetch_morph_files(plan['id'])
            i = 0
            for motion_file, morph_file in zip_longest(motion_files, morph_files):
                if motion_file:
                    motion_file_path = self.process_path(motion_file, export_path, export_type)
                    playlist.append(f"Dancer{i}Motion={motion_file_path}")
                if morph_file:
                    morph_file_path = self.process_path(morph_file, export_path, export_type)
                    playlist.append(f"Dancer{i}Morph={morph_file_path}")
                i += 1
            camera_files = self.performancePlanModel.fetch_camera_files(plan['id'])
            i = 0
            for camera_file in camera_files:
                camera_file_path = self.process_path(camera_file, export_path, export_type)
                playlist.append(f"Camera{i}={camera_file_path}")
                i+1
            music_files = self.performancePlanModel.fetch_music_files(plan['id'])
            i = 0
            for music_file in music_files:
                music_file_path = self.process_path(music_file, export_path, export_type)
                playlist.append(f"Music{i}={music_file_path}")
                i+1   

            playlist.append("") 
        playlist_file = "\n".join(playlist)
        playlist_file_path = os.path.join(export_path, playlist_name+'.txt')
        playlist_file_path = os.path.normpath(playlist_file_path)
        # 检查文件是否存在
        if os.path.exists(playlist_file_path):
            raise FileExistsError(f"The file '{playlist_file_path}' already exists.")

        # 如果文件不存在，正常写入数据
        with open(playlist_file_path, 'w', encoding='utf-8') as file:
            file.write(playlist_file)
        utils.log_and_print(f"playlist:{playlist_file_path} exported")

    def process_path(self, file, export_path, export_type):
        file_rel_path = os.path.join(file.folder_path, file.name)
        if export_path == FileDatabasePath and export_type == 'rel':
            return file_rel_path
        file_abs_path = os.path.join(FileDatabasePath, file_rel_path)
        if export_type == 'abs':
            return file_abs_path
        file_rel_export_path= os.path.relpath(file_abs_path, export_path)
        return file_rel_export_path

    @staticmethod
    def get_list_data(list):
        data_list = []
        if isinstance(list, QListWidget):
            for i in range(list.count()):
                item = list.item(i)
                data = item.data(Qt.UserRole) 
                data_list.append(data)
        elif isinstance(list, QListView):
            model = list.model()
            if model:
                for row in range(model.rowCount()):
                    index = model.index(row, 0)  # 假设ID存储在第一列
                    data = model.data(index, Qt.ItemDataRole.UserRole)
                    if data:
                        data_list.append(data)
        return data_list

    @staticmethod
    def set_filter(filter_type, new_filter, lineEdit):
        # 获取当前lineEdit中的文本，并按分号分割成列表
        filters = lineEdit.text().split("; ") if lineEdit.text() else []
        filter_text = filter_type + ":" + new_filter

        if filter_text in filters:
            filters.remove(filter_text)
        else:
            # 处理每个过滤器，检查是否有相同类型的过滤器存在
            updated = False
            for i in range(len(filters)):
                if filters[i].startswith(filter_type + ":"):
                    # 如果找到相同类型的过滤器，更新它
                    filters[i] = filter_type + ":" + new_filter
                    updated = True
                    break

            # 如果没有找到相同类型的过滤器，添加新的过滤器
            if not updated:
                filters.append(filter_text)

        # 将更新后的过滤器列表转换回字符串，用分号空格分隔，然后设置回lineEdit
        lineEdit.setText("; ".join(filters))

    @staticmethod
    def parse_fliter(data_string):
        entries = data_string.split('; ')if data_string else []
        result = {}
        for entry in entries:
            if not entry:
                continue
            parts = entry.split('=')
            if len(parts) == 2:
                type_id, name = parts
                type_part, id_part = type_id.split(':')
                result[type_part.strip()] = id_part.strip()
        return result

    def remove_listWidget_duplicates(self, listWidget):
        seen = {}
        index = 0
        # 从后向前遍历，以便删除时不影响索引
        while index < listWidget.count():
            item = listWidget.item(index)
            name = item.text()
            if name in seen:
                # 如果已经见过，删除
                item = listWidget.takeItem(index)
                # 不增加索引
                continue
            else:
                # 如果没见过，记录并增加索引
                seen[name] = True
            index += 1

    # 刷新所有ppd listwidget内容
    def reset_all_ppd_listWSidget(self):
        self.ui.pp_ppd_dancer.clear()
        self.ui.pp_ppd_camera.clear()
        self.ui.pp_ppd_music.clear()
        self.ui.pp_ppd_tags.clear()

    def reset_all_ppd(self):
        self.ui.pp_ppd_plan_id.setText('0')
        self.ui.pp_ppd_plan_name.setText('')
        self.ui.pp_ppd_dancer_number.setValue(0)
        self.ui.pp_ppd_rating.setValue(0)
        self.ui.pp_ppd_comments.setText('无评论')
        self.reset_all_ppd_listWSidget()

    def reset_all_playlist(self):
        self.ui.pp_playlist_id.setText('0')
        self.ui.pp_playlist_name.setText('')
        self.ui.pp_performance_plan_in_playlist.clear()
        self.update_search(self.playlistModel, search_text=self.ui.pp_playlist_search.text())

    # 刷新所有模型
    def refresh_all_models(self, lineEdit=None):
        if lineEdit == None:
            lineEdit = self.ui.pp_filter
        filters = self.parse_fliter(lineEdit.text()) 
        self.update_search(self.performancePlanModel, search_text=self.ui.pp_performance_plan_search.text(), filters=filters)
        self.update_search(self.authorModel, search_text=self.ui.pp_song_search.text(), filters=filters)
        self.update_search(self.songModel, search_text=self.ui.pp_author_search.text(), filters=filters)
        self.update_search(self.tagModel, search_text=self.ui.pp_tag_search.text())
        self.update_search(self.playlistModel, search_text=self.ui.pp_playlist_search.text())
        self.fileModel_dancer.fetch_data()
        self.fileModel_dancer_motion.fetch_data()
        self.fileModel_dancer_morph.fetch_data()
        self.fileModel_camera.fetch_data()
        self.fileModel_music.fetch_data()

    def restart_window(self):
        self.close()  # 关闭当前窗口
        self.new_window = MainWindow(self.session)  # 假设MainWindow是你的窗口类
        self.new_window.show()

    # 刷新搜索
    def update_search(self, model, search_text='', filters=None, fetch_size=None):
        model.fetch_data(search_text=search_text, filters=filters, fetch_size=fetch_size, reset=True)

    #确保ui维持上次关闭的样子
    def find_splitters(self, widget):
        """递归查找所有QSplitter控件"""
        splitters = []
        if isinstance(widget, QSplitter):
            splitters.append(widget)
        for child in widget.children():
            splitters.extend(self.find_splitters(child))
        return splitters

    def closeEvent(self, event: QEvent):
        reply = QMessageBox.question(self, '确认关闭',
                                     "你确定要关闭窗口吗?",
                                     QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No,
                                     QMessageBox.StandardButton.No)
        if reply == QMessageBox.StandardButton.Yes:
            self.saveUIState()
            utils.log_and_print('窗口即将关闭...')
            if flushed_but_not_committed:
                reply_database = QMessageBox.question(self, '可能有未保存的内容',
                                                      "你确定要保存数据库吗?",
                                                      QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No,
                                                      QMessageBox.StandardButton.No)
                if reply_database == QMessageBox.StandardButton.Yes:
                    self.commit_database()
            self.session.close()
            super().closeEvent(event)
            event.accept()  # 接受关闭事件，关闭窗口
        else:
            event.ignore()  # 忽略关闭事件，窗口不会关闭

    def initUIState(self):
        settings = QSettings("uisettings.ini", QSettings.Format.IniFormat)
        self.restoreGeometry(settings.value("geometry", type=bytes))
        for splitter in self.find_splitters(self):
            key = f"splitterState_{splitter.objectName()}"
            state = settings.value(key, type=bytes)
            if state:
                splitter.restoreState(state)

    def saveUIState(self):
        settings = QSettings("uisettings.ini", QSettings.Format.IniFormat)
        settings.setValue("geometry", self.saveGeometry())
        for splitter in self.find_splitters(self):
            key = f"splitterState_{splitter.objectName()}"
            settings.setValue(key, splitter.saveState())

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, filename='log.log', filemode='a', format='%(asctime)s - %(levelname)s - %(message)s')
    # 读取配置文件
    config = configparser.ConfigParser()
    with open('../config.ini', 'r', encoding='utf-8') as configfile:
        config.read_file(configfile)

    # 获取路径信息
    SQLDatabasePath = os.path.abspath(config['Paths']['SQLDatabasePath'])
    FileDatabasePath = os.path.abspath(config['Paths']['FileDatabasePath'])
    print('SQLDatabasePath:',SQLDatabasePath)
    print('FileDatabasePath:',FileDatabasePath)
    SelectiveScanFolder = os.path.abspath(config['Paths']['SelectiveScanFolder'])
    ScanAndDeleteDuplicateFolder = os.path.abspath(config['Paths']['ScanAndDeleteDuplicateFolder'])
    
    FolderFormat = config['Format']['FolderFormat']
    SongNameFormat = config['Format']['SongNameFormat']

    ExportPath =  config['Export']['ExportPath']
    ExportType = config['Export']['ExportType']

    RearrangeFolder = os.path.abspath(config['Rearrange']['RearrangeFolder'])
    RearrangeFormat = config['Rearrange']['RearrangeFormat']

    # 检查数据库文件是否存在
    db_exists = os.path.exists(SQLDatabasePath)

    # 创建数据库引擎
    engine = create_engine(f'sqlite:///{SQLDatabasePath}')

    # 如果数据库文件不存在，则创建数据库表
    if not db_exists:
        Base.metadata.create_all(engine)

    # 创建一个会话
    Session = sessionmaker(bind=engine)
    session = Session()
    
    # 标志位来检查状态
    flushed_but_not_committed = False

    @event.listens_for(Session, 'after_flush')
    def receive_after_flush(session, flush_context):
        global flushed_but_not_committed
        flushed_but_not_committed = True

    @event.listens_for(Session, 'after_commit')
    def receive_after_commit(session):
        global flushed_but_not_committed
        flushed_but_not_committed = False

    app = QApplication(sys.argv)
    # 创建主窗口实例
    main_window = MainWindow(session)
    # 显示主窗口
    main_window.show()
    # 运行应用程序
    sys.exit(app.exec())









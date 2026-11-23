# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'VmdManagerhNepAr.ui'
##
## Created by: Qt User Interface Compiler version 6.4.3
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QAction, QBrush, QColor, QConicalGradient,
    QCursor, QFont, QFontDatabase, QGradient,
    QIcon, QImage, QKeySequence, QLinearGradient,
    QPainter, QPalette, QPixmap, QRadialGradient,
    QTransform)
from PySide6.QtWidgets import (QAbstractItemView, QApplication, QCheckBox, QComboBox,
    QGroupBox, QHBoxLayout, QLabel, QLineEdit,
    QListView, QListWidget, QListWidgetItem, QMainWindow,
    QMenu, QMenuBar, QPlainTextEdit, QPushButton,
    QSizePolicy, QSpinBox, QSplitter, QStatusBar,
    QTabWidget, QTextEdit, QVBoxLayout, QWidget)

class Ui_mainWindow(object):
    def setupUi(self, mainWindow):
        if not mainWindow.objectName():
            mainWindow.setObjectName(u"mainWindow")
        mainWindow.resize(1067, 788)
        mainWindow.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.actionScanFileDatabasePath = QAction(mainWindow)
        self.actionScanFileDatabasePath.setObjectName(u"actionScanFileDatabasePath")
        self.actionBackupDatabase = QAction(mainWindow)
        self.actionBackupDatabase.setObjectName(u"actionBackupDatabase")
        self.actionSetting = QAction(mainWindow)
        self.actionSetting.setObjectName(u"actionSetting")
        self.actionRestartWindow = QAction(mainWindow)
        self.actionRestartWindow.setObjectName(u"actionRestartWindow")
        self.actionResetPPD = QAction(mainWindow)
        self.actionResetPPD.setObjectName(u"actionResetPPD")
        self.actionResetPlaylist = QAction(mainWindow)
        self.actionResetPlaylist.setObjectName(u"actionResetPlaylist")
        self.actionCleanDatabase = QAction(mainWindow)
        self.actionCleanDatabase.setObjectName(u"actionCleanDatabase")
        self.actionCleanSongSingerAuthor = QAction(mainWindow)
        self.actionCleanSongSingerAuthor.setObjectName(u"actionCleanSongSingerAuthor")
        self.centralwidget = QWidget(mainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.verticalLayout_20 = QVBoxLayout(self.centralwidget)
        self.verticalLayout_20.setObjectName(u"verticalLayout_20")
        self.tabWidget = QTabWidget(self.centralwidget)
        self.tabWidget.setObjectName(u"tabWidget")
        self.tab_1 = QWidget()
        self.tab_1.setObjectName(u"tab_1")
        self.verticalLayout = QVBoxLayout(self.tab_1)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.splitter = QSplitter(self.tab_1)
        self.splitter.setObjectName(u"splitter")
        self.splitter.setOrientation(Qt.Horizontal)
        self.widget = QWidget(self.splitter)
        self.widget.setObjectName(u"widget")
        self.verticalLayout_2 = QVBoxLayout(self.widget)
        self.verticalLayout_2.setObjectName(u"verticalLayout_2")
        self.splitter_2 = QSplitter(self.widget)
        self.splitter_2.setObjectName(u"splitter_2")
        self.splitter_2.setOrientation(Qt.Vertical)
        self.widget_2 = QWidget(self.splitter_2)
        self.widget_2.setObjectName(u"widget_2")
        self.horizontalLayout = QHBoxLayout(self.widget_2)
        self.horizontalLayout.setObjectName(u"horizontalLayout")
        self.splitter_3 = QSplitter(self.widget_2)
        self.splitter_3.setObjectName(u"splitter_3")
        self.splitter_3.setOrientation(Qt.Horizontal)
        self.groupBox_3 = QGroupBox(self.splitter_3)
        self.groupBox_3.setObjectName(u"groupBox_3")
        self.verticalLayout_6 = QVBoxLayout(self.groupBox_3)
        self.verticalLayout_6.setObjectName(u"verticalLayout_6")
        self.pp_performance_plan = QListView(self.groupBox_3)
        self.pp_performance_plan.setObjectName(u"pp_performance_plan")
        self.pp_performance_plan.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_performance_plan.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_6.addWidget(self.pp_performance_plan)

        self.widget_3 = QWidget(self.groupBox_3)
        self.widget_3.setObjectName(u"widget_3")
        self.horizontalLayout_20 = QHBoxLayout(self.widget_3)
        self.horizontalLayout_20.setObjectName(u"horizontalLayout_20")
        self.pp_performance_plan_search = QLineEdit(self.widget_3)
        self.pp_performance_plan_search.setObjectName(u"pp_performance_plan_search")

        self.horizontalLayout_20.addWidget(self.pp_performance_plan_search)


        self.verticalLayout_6.addWidget(self.widget_3)

        self.splitter_3.addWidget(self.groupBox_3)
        self.widget_16 = QWidget(self.splitter_3)
        self.widget_16.setObjectName(u"widget_16")
        self.verticalLayout_8 = QVBoxLayout(self.widget_16)
        self.verticalLayout_8.setObjectName(u"verticalLayout_8")
        self.tabWidget_3 = QTabWidget(self.widget_16)
        self.tabWidget_3.setObjectName(u"tabWidget_3")
        self.tab_9 = QWidget()
        self.tab_9.setObjectName(u"tab_9")
        self.verticalLayout_7 = QVBoxLayout(self.tab_9)
        self.verticalLayout_7.setObjectName(u"verticalLayout_7")
        self.verticalLayout_7.setContentsMargins(9, 9, 9, 9)
        self.pp_song = QListView(self.tab_9)
        self.pp_song.setObjectName(u"pp_song")
        self.pp_song.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_7.addWidget(self.pp_song)

        self.pp_song_search = QLineEdit(self.tab_9)
        self.pp_song_search.setObjectName(u"pp_song_search")

        self.verticalLayout_7.addWidget(self.pp_song_search)

        self.tabWidget_3.addTab(self.tab_9, "")
        self.tab_10 = QWidget()
        self.tab_10.setObjectName(u"tab_10")
        self.verticalLayout_16 = QVBoxLayout(self.tab_10)
        self.verticalLayout_16.setObjectName(u"verticalLayout_16")
        self.pp_author = QListView(self.tab_10)
        self.pp_author.setObjectName(u"pp_author")
        self.pp_author.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_16.addWidget(self.pp_author)

        self.pp_author_search = QLineEdit(self.tab_10)
        self.pp_author_search.setObjectName(u"pp_author_search")

        self.verticalLayout_16.addWidget(self.pp_author_search)

        self.tabWidget_3.addTab(self.tab_10, "")
        self.tab_11 = QWidget()
        self.tab_11.setObjectName(u"tab_11")
        self.verticalLayout_3 = QVBoxLayout(self.tab_11)
        self.verticalLayout_3.setObjectName(u"verticalLayout_3")
        self.pp_tag = QListView(self.tab_11)
        self.pp_tag.setObjectName(u"pp_tag")
        self.pp_tag.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_3.addWidget(self.pp_tag)

        self.pp_tag_search = QLineEdit(self.tab_11)
        self.pp_tag_search.setObjectName(u"pp_tag_search")

        self.verticalLayout_3.addWidget(self.pp_tag_search)

        self.pp_delete_tag = QPushButton(self.tab_11)
        self.pp_delete_tag.setObjectName(u"pp_delete_tag")

        self.verticalLayout_3.addWidget(self.pp_delete_tag)

        self.tabWidget_3.addTab(self.tab_11, "")

        self.verticalLayout_8.addWidget(self.tabWidget_3)

        self.widget_8 = QWidget(self.widget_16)
        self.widget_8.setObjectName(u"widget_8")
        self.horizontalLayout_4 = QHBoxLayout(self.widget_8)
        self.horizontalLayout_4.setObjectName(u"horizontalLayout_4")
        self.horizontalLayout_4.setContentsMargins(2, 1, 2, 1)
        self.pp_filter = QLineEdit(self.widget_8)
        self.pp_filter.setObjectName(u"pp_filter")
        self.pp_filter.setReadOnly(True)

        self.horizontalLayout_4.addWidget(self.pp_filter)


        self.verticalLayout_8.addWidget(self.widget_8)

        self.splitter_3.addWidget(self.widget_16)

        self.horizontalLayout.addWidget(self.splitter_3)

        self.splitter_2.addWidget(self.widget_2)
        self.groupBox_2 = QGroupBox(self.splitter_2)
        self.groupBox_2.setObjectName(u"groupBox_2")
        self.horizontalLayout_12 = QHBoxLayout(self.groupBox_2)
        self.horizontalLayout_12.setObjectName(u"horizontalLayout_12")
        self.splitter_8 = QSplitter(self.groupBox_2)
        self.splitter_8.setObjectName(u"splitter_8")
        self.splitter_8.setOrientation(Qt.Horizontal)
        self.widget_11 = QWidget(self.splitter_8)
        self.widget_11.setObjectName(u"widget_11")
        self.verticalLayout_11 = QVBoxLayout(self.widget_11)
        self.verticalLayout_11.setObjectName(u"verticalLayout_11")
        self.splitter_9 = QSplitter(self.widget_11)
        self.splitter_9.setObjectName(u"splitter_9")
        self.splitter_9.setOrientation(Qt.Vertical)
        self.widget_6 = QWidget(self.splitter_9)
        self.widget_6.setObjectName(u"widget_6")
        self.verticalLayout_9 = QVBoxLayout(self.widget_6)
        self.verticalLayout_9.setObjectName(u"verticalLayout_9")
        self.verticalLayout_9.setContentsMargins(1, 1, 1, 1)
        self.widget_13 = QWidget(self.widget_6)
        self.widget_13.setObjectName(u"widget_13")
        self.horizontalLayout_19 = QHBoxLayout(self.widget_13)
        self.horizontalLayout_19.setSpacing(31)
        self.horizontalLayout_19.setObjectName(u"horizontalLayout_19")
        self.horizontalLayout_19.setContentsMargins(1, 1, 1, 1)
        self.label_5 = QLabel(self.widget_13)
        self.label_5.setObjectName(u"label_5")

        self.horizontalLayout_19.addWidget(self.label_5)

        self.pp_ppd_plan_id = QLineEdit(self.widget_13)
        self.pp_ppd_plan_id.setObjectName(u"pp_ppd_plan_id")
        self.pp_ppd_plan_id.setAlignment(Qt.AlignRight|Qt.AlignTrailing|Qt.AlignVCenter)
        self.pp_ppd_plan_id.setReadOnly(True)

        self.horizontalLayout_19.addWidget(self.pp_ppd_plan_id)


        self.verticalLayout_9.addWidget(self.widget_13)

        self.widget_7 = QWidget(self.widget_6)
        self.widget_7.setObjectName(u"widget_7")
        self.horizontalLayout_3 = QHBoxLayout(self.widget_7)
        self.horizontalLayout_3.setSpacing(8)
        self.horizontalLayout_3.setObjectName(u"horizontalLayout_3")
        self.horizontalLayout_3.setContentsMargins(1, 1, 1, 1)
        self.label = QLabel(self.widget_7)
        self.label.setObjectName(u"label")

        self.horizontalLayout_3.addWidget(self.label)

        self.pp_ppd_plan_name = QLineEdit(self.widget_7)
        self.pp_ppd_plan_name.setObjectName(u"pp_ppd_plan_name")

        self.horizontalLayout_3.addWidget(self.pp_ppd_plan_name)


        self.verticalLayout_9.addWidget(self.widget_7)

        self.widget_9 = QWidget(self.widget_6)
        self.widget_9.setObjectName(u"widget_9")
        self.horizontalLayout_5 = QHBoxLayout(self.widget_9)
        self.horizontalLayout_5.setObjectName(u"horizontalLayout_5")
        self.horizontalLayout_5.setContentsMargins(1, 1, 1, 1)
        self.label_3 = QLabel(self.widget_9)
        self.label_3.setObjectName(u"label_3")

        self.horizontalLayout_5.addWidget(self.label_3)

        self.pp_ppd_dancer_number = QSpinBox(self.widget_9)
        self.pp_ppd_dancer_number.setObjectName(u"pp_ppd_dancer_number")
        self.pp_ppd_dancer_number.setAlignment(Qt.AlignRight|Qt.AlignTrailing|Qt.AlignVCenter)
        self.pp_ppd_dancer_number.setReadOnly(True)

        self.horizontalLayout_5.addWidget(self.pp_ppd_dancer_number)


        self.verticalLayout_9.addWidget(self.widget_9)

        self.splitter_9.addWidget(self.widget_6)
        self.groupBox_11 = QGroupBox(self.splitter_9)
        self.groupBox_11.setObjectName(u"groupBox_11")
        self.horizontalLayout_15 = QHBoxLayout(self.groupBox_11)
        self.horizontalLayout_15.setObjectName(u"horizontalLayout_15")
        self.horizontalLayout_15.setContentsMargins(1, 1, 1, 1)
        self.pp_ppd_dancer = QListWidget(self.groupBox_11)
        self.pp_ppd_dancer.setObjectName(u"pp_ppd_dancer")
        self.pp_ppd_dancer.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_ppd_dancer.setDragEnabled(True)
        self.pp_ppd_dancer.setDragDropMode(QAbstractItemView.InternalMove)
        self.pp_ppd_dancer.setDefaultDropAction(Qt.MoveAction)
        self.pp_ppd_dancer.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_15.addWidget(self.pp_ppd_dancer)

        self.splitter_9.addWidget(self.groupBox_11)

        self.verticalLayout_11.addWidget(self.splitter_9)

        self.widget_14 = QWidget(self.widget_11)
        self.widget_14.setObjectName(u"widget_14")
        self.horizontalLayout_21 = QHBoxLayout(self.widget_14)
        self.horizontalLayout_21.setSpacing(5)
        self.horizontalLayout_21.setObjectName(u"horizontalLayout_21")
        self.horizontalLayout_21.setContentsMargins(1, 1, 1, 1)
        self.pp_save_plan = QPushButton(self.widget_14)
        self.pp_save_plan.setObjectName(u"pp_save_plan")

        self.horizontalLayout_21.addWidget(self.pp_save_plan)

        self.pp_new_plan = QPushButton(self.widget_14)
        self.pp_new_plan.setObjectName(u"pp_new_plan")

        self.horizontalLayout_21.addWidget(self.pp_new_plan)

        self.pp_delete_plan = QPushButton(self.widget_14)
        self.pp_delete_plan.setObjectName(u"pp_delete_plan")
        self.pp_delete_plan.setCheckable(False)

        self.horizontalLayout_21.addWidget(self.pp_delete_plan)


        self.verticalLayout_11.addWidget(self.widget_14)

        self.splitter_8.addWidget(self.widget_11)
        self.tabWidget_10 = QTabWidget(self.splitter_8)
        self.tabWidget_10.setObjectName(u"tabWidget_10")
        self.tabWidget_10Page1 = QWidget()
        self.tabWidget_10Page1.setObjectName(u"tabWidget_10Page1")
        self.verticalLayout_10 = QVBoxLayout(self.tabWidget_10Page1)
        self.verticalLayout_10.setObjectName(u"verticalLayout_10")
        self.splitter_10 = QSplitter(self.tabWidget_10Page1)
        self.splitter_10.setObjectName(u"splitter_10")
        self.splitter_10.setOrientation(Qt.Vertical)
        self.groupBox_12 = QGroupBox(self.splitter_10)
        self.groupBox_12.setObjectName(u"groupBox_12")
        self.horizontalLayout_13 = QHBoxLayout(self.groupBox_12)
        self.horizontalLayout_13.setObjectName(u"horizontalLayout_13")
        self.pp_ppd_camera = QListWidget(self.groupBox_12)
        self.pp_ppd_camera.setObjectName(u"pp_ppd_camera")
        self.pp_ppd_camera.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_ppd_camera.setDragEnabled(True)
        self.pp_ppd_camera.setDragDropMode(QAbstractItemView.InternalMove)
        self.pp_ppd_camera.setDefaultDropAction(Qt.MoveAction)
        self.pp_ppd_camera.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_13.addWidget(self.pp_ppd_camera)

        self.splitter_10.addWidget(self.groupBox_12)
        self.groupBox_13 = QGroupBox(self.splitter_10)
        self.groupBox_13.setObjectName(u"groupBox_13")
        self.verticalLayout_12 = QVBoxLayout(self.groupBox_13)
        self.verticalLayout_12.setObjectName(u"verticalLayout_12")
        self.pp_ppd_music = QListWidget(self.groupBox_13)
        self.pp_ppd_music.setObjectName(u"pp_ppd_music")
        self.pp_ppd_music.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_ppd_music.setDragEnabled(True)
        self.pp_ppd_music.setDragDropMode(QAbstractItemView.InternalMove)
        self.pp_ppd_music.setDefaultDropAction(Qt.MoveAction)
        self.pp_ppd_music.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_12.addWidget(self.pp_ppd_music)

        self.splitter_10.addWidget(self.groupBox_13)

        self.verticalLayout_10.addWidget(self.splitter_10)

        self.tabWidget_10.addTab(self.tabWidget_10Page1, "")
        self.tabWidget_10Page2 = QWidget()
        self.tabWidget_10Page2.setObjectName(u"tabWidget_10Page2")
        self.verticalLayout_13 = QVBoxLayout(self.tabWidget_10Page2)
        self.verticalLayout_13.setObjectName(u"verticalLayout_13")
        self.widget_10 = QWidget(self.tabWidget_10Page2)
        self.widget_10.setObjectName(u"widget_10")
        self.horizontalLayout_14 = QHBoxLayout(self.widget_10)
        self.horizontalLayout_14.setObjectName(u"horizontalLayout_14")
        self.label_4 = QLabel(self.widget_10)
        self.label_4.setObjectName(u"label_4")

        self.horizontalLayout_14.addWidget(self.label_4)

        self.pp_ppd_rating = QSpinBox(self.widget_10)
        self.pp_ppd_rating.setObjectName(u"pp_ppd_rating")
        self.pp_ppd_rating.setMinimum(0)
        self.pp_ppd_rating.setMaximum(10)
        self.pp_ppd_rating.setValue(0)

        self.horizontalLayout_14.addWidget(self.pp_ppd_rating)


        self.verticalLayout_13.addWidget(self.widget_10)

        self.groupBox_14 = QGroupBox(self.tabWidget_10Page2)
        self.groupBox_14.setObjectName(u"groupBox_14")
        self.verticalLayout_14 = QVBoxLayout(self.groupBox_14)
        self.verticalLayout_14.setObjectName(u"verticalLayout_14")
        self.pp_ppd_tags = QListWidget(self.groupBox_14)
        self.pp_ppd_tags.setObjectName(u"pp_ppd_tags")
        self.pp_ppd_tags.setContextMenuPolicy(Qt.CustomContextMenu)

        self.verticalLayout_14.addWidget(self.pp_ppd_tags)

        self.widget_12 = QWidget(self.groupBox_14)
        self.widget_12.setObjectName(u"widget_12")
        self.horizontalLayout_18 = QHBoxLayout(self.widget_12)
        self.horizontalLayout_18.setSpacing(3)
        self.horizontalLayout_18.setObjectName(u"horizontalLayout_18")
        self.horizontalLayout_18.setContentsMargins(1, 1, 1, 1)
        self.pp_ppd_tag_comboBox = QComboBox(self.widget_12)
        self.pp_ppd_tag_comboBox.setObjectName(u"pp_ppd_tag_comboBox")
        self.pp_ppd_tag_comboBox.setEditable(True)
        self.pp_ppd_tag_comboBox.setDuplicatesEnabled(True)

        self.horizontalLayout_18.addWidget(self.pp_ppd_tag_comboBox)

        self.pp_ppd_add_tag = QPushButton(self.widget_12)
        self.pp_ppd_add_tag.setObjectName(u"pp_ppd_add_tag")

        self.horizontalLayout_18.addWidget(self.pp_ppd_add_tag)

        self.horizontalLayout_18.setStretch(0, 3)
        self.horizontalLayout_18.setStretch(1, 1)

        self.verticalLayout_14.addWidget(self.widget_12)


        self.verticalLayout_13.addWidget(self.groupBox_14)

        self.tabWidget_10.addTab(self.tabWidget_10Page2, "")
        self.tab_3 = QWidget()
        self.tab_3.setObjectName(u"tab_3")
        self.horizontalLayout_17 = QHBoxLayout(self.tab_3)
        self.horizontalLayout_17.setObjectName(u"horizontalLayout_17")
        self.pp_ppd_comments = QTextEdit(self.tab_3)
        self.pp_ppd_comments.setObjectName(u"pp_ppd_comments")
        self.pp_ppd_comments.setContextMenuPolicy(Qt.DefaultContextMenu)

        self.horizontalLayout_17.addWidget(self.pp_ppd_comments)

        self.tabWidget_10.addTab(self.tab_3, "")
        self.splitter_8.addWidget(self.tabWidget_10)

        self.horizontalLayout_12.addWidget(self.splitter_8)

        self.splitter_2.addWidget(self.groupBox_2)

        self.verticalLayout_2.addWidget(self.splitter_2)

        self.splitter.addWidget(self.widget)
        self.tabWidget1 = QTabWidget(self.splitter)
        self.tabWidget1.setObjectName(u"tabWidget1")
        self.tabWidgetPage1 = QWidget()
        self.tabWidgetPage1.setObjectName(u"tabWidgetPage1")
        self.horizontalLayout_6 = QHBoxLayout(self.tabWidgetPage1)
        self.horizontalLayout_6.setObjectName(u"horizontalLayout_6")
        self.splitter_5 = QSplitter(self.tabWidgetPage1)
        self.splitter_5.setObjectName(u"splitter_5")
        self.splitter_5.setOrientation(Qt.Horizontal)
        self.widget_4 = QWidget(self.splitter_5)
        self.widget_4.setObjectName(u"widget_4")
        self.verticalLayout_4 = QVBoxLayout(self.widget_4)
        self.verticalLayout_4.setObjectName(u"verticalLayout_4")
        self.splitter_6 = QSplitter(self.widget_4)
        self.splitter_6.setObjectName(u"splitter_6")
        self.splitter_6.setOrientation(Qt.Vertical)
        self.groupBox_6 = QGroupBox(self.splitter_6)
        self.groupBox_6.setObjectName(u"groupBox_6")
        self.horizontalLayout_7 = QHBoxLayout(self.groupBox_6)
        self.horizontalLayout_7.setObjectName(u"horizontalLayout_7")
        self.pp_file_dancer = QListView(self.groupBox_6)
        self.pp_file_dancer.setObjectName(u"pp_file_dancer")
        self.pp_file_dancer.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_file_dancer.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_7.addWidget(self.pp_file_dancer)

        self.splitter_6.addWidget(self.groupBox_6)
        self.groupBox_7 = QGroupBox(self.splitter_6)
        self.groupBox_7.setObjectName(u"groupBox_7")
        self.horizontalLayout_8 = QHBoxLayout(self.groupBox_7)
        self.horizontalLayout_8.setObjectName(u"horizontalLayout_8")
        self.pp_file_camera = QListView(self.groupBox_7)
        self.pp_file_camera.setObjectName(u"pp_file_camera")
        self.pp_file_camera.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_file_camera.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_8.addWidget(self.pp_file_camera)

        self.splitter_6.addWidget(self.groupBox_7)

        self.verticalLayout_4.addWidget(self.splitter_6)

        self.splitter_5.addWidget(self.widget_4)
        self.widget_5 = QWidget(self.splitter_5)
        self.widget_5.setObjectName(u"widget_5")
        self.verticalLayout_5 = QVBoxLayout(self.widget_5)
        self.verticalLayout_5.setObjectName(u"verticalLayout_5")
        self.splitter_7 = QSplitter(self.widget_5)
        self.splitter_7.setObjectName(u"splitter_7")
        self.splitter_7.setOrientation(Qt.Vertical)
        self.groupBox_8 = QGroupBox(self.splitter_7)
        self.groupBox_8.setObjectName(u"groupBox_8")
        self.horizontalLayout_9 = QHBoxLayout(self.groupBox_8)
        self.horizontalLayout_9.setObjectName(u"horizontalLayout_9")
        self.pp_file_dancer_motion = QListView(self.groupBox_8)
        self.pp_file_dancer_motion.setObjectName(u"pp_file_dancer_motion")
        self.pp_file_dancer_motion.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_9.addWidget(self.pp_file_dancer_motion)

        self.splitter_7.addWidget(self.groupBox_8)
        self.groupBox_9 = QGroupBox(self.splitter_7)
        self.groupBox_9.setObjectName(u"groupBox_9")
        self.horizontalLayout_10 = QHBoxLayout(self.groupBox_9)
        self.horizontalLayout_10.setObjectName(u"horizontalLayout_10")
        self.pp_file_dancer_morph = QListView(self.groupBox_9)
        self.pp_file_dancer_morph.setObjectName(u"pp_file_dancer_morph")
        self.pp_file_dancer_morph.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_10.addWidget(self.pp_file_dancer_morph)

        self.splitter_7.addWidget(self.groupBox_9)
        self.groupBox_10 = QGroupBox(self.splitter_7)
        self.groupBox_10.setObjectName(u"groupBox_10")
        self.horizontalLayout_11 = QHBoxLayout(self.groupBox_10)
        self.horizontalLayout_11.setObjectName(u"horizontalLayout_11")
        self.pp_file_music = QListView(self.groupBox_10)
        self.pp_file_music.setObjectName(u"pp_file_music")
        self.pp_file_music.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.horizontalLayout_11.addWidget(self.pp_file_music)

        self.splitter_7.addWidget(self.groupBox_10)

        self.verticalLayout_5.addWidget(self.splitter_7)

        self.splitter_5.addWidget(self.widget_5)

        self.horizontalLayout_6.addWidget(self.splitter_5)

        self.tabWidget1.addTab(self.tabWidgetPage1, "")
        self.tab_7 = QWidget()
        self.tab_7.setObjectName(u"tab_7")
        self.horizontalLayout_16 = QHBoxLayout(self.tab_7)
        self.horizontalLayout_16.setObjectName(u"horizontalLayout_16")
        self.splitter_4 = QSplitter(self.tab_7)
        self.splitter_4.setObjectName(u"splitter_4")
        self.splitter_4.setOrientation(Qt.Horizontal)
        self.groupBox = QGroupBox(self.splitter_4)
        self.groupBox.setObjectName(u"groupBox")
        self.verticalLayout_17 = QVBoxLayout(self.groupBox)
        self.verticalLayout_17.setObjectName(u"verticalLayout_17")
        self.pp_performance_plan_in_playlist = QListWidget(self.groupBox)
        self.pp_performance_plan_in_playlist.setObjectName(u"pp_performance_plan_in_playlist")
        self.pp_performance_plan_in_playlist.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_performance_plan_in_playlist.setDragEnabled(True)
        self.pp_performance_plan_in_playlist.setDragDropMode(QAbstractItemView.InternalMove)
        self.pp_performance_plan_in_playlist.setDefaultDropAction(Qt.MoveAction)
        self.pp_performance_plan_in_playlist.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_17.addWidget(self.pp_performance_plan_in_playlist)

        self.pp_add_filtered_plan = QPushButton(self.groupBox)
        self.pp_add_filtered_plan.setObjectName(u"pp_add_filtered_plan")

        self.verticalLayout_17.addWidget(self.pp_add_filtered_plan)

        self.pp_remove_duplicate_entries = QPushButton(self.groupBox)
        self.pp_remove_duplicate_entries.setObjectName(u"pp_remove_duplicate_entries")

        self.verticalLayout_17.addWidget(self.pp_remove_duplicate_entries)

        self.splitter_4.addWidget(self.groupBox)
        self.groupBox_4 = QGroupBox(self.splitter_4)
        self.groupBox_4.setObjectName(u"groupBox_4")
        self.verticalLayout_15 = QVBoxLayout(self.groupBox_4)
        self.verticalLayout_15.setObjectName(u"verticalLayout_15")
        self.pp_playlist = QListView(self.groupBox_4)
        self.pp_playlist.setObjectName(u"pp_playlist")
        self.pp_playlist.setContextMenuPolicy(Qt.DefaultContextMenu)
        self.pp_playlist.setSelectionMode(QAbstractItemView.ExtendedSelection)

        self.verticalLayout_15.addWidget(self.pp_playlist)

        self.pp_playlist_search = QLineEdit(self.groupBox_4)
        self.pp_playlist_search.setObjectName(u"pp_playlist_search")

        self.verticalLayout_15.addWidget(self.pp_playlist_search)

        self.widget_17 = QWidget(self.groupBox_4)
        self.widget_17.setObjectName(u"widget_17")
        self.horizontalLayout_23 = QHBoxLayout(self.widget_17)
        self.horizontalLayout_23.setSpacing(28)
        self.horizontalLayout_23.setObjectName(u"horizontalLayout_23")
        self.horizontalLayout_23.setContentsMargins(1, 1, 1, 1)
        self.label_6 = QLabel(self.widget_17)
        self.label_6.setObjectName(u"label_6")

        self.horizontalLayout_23.addWidget(self.label_6)

        self.pp_playlist_id = QLineEdit(self.widget_17)
        self.pp_playlist_id.setObjectName(u"pp_playlist_id")
        self.pp_playlist_id.setAlignment(Qt.AlignRight|Qt.AlignTrailing|Qt.AlignVCenter)
        self.pp_playlist_id.setReadOnly(True)

        self.horizontalLayout_23.addWidget(self.pp_playlist_id)


        self.verticalLayout_15.addWidget(self.widget_17)

        self.widget_15 = QWidget(self.groupBox_4)
        self.widget_15.setObjectName(u"widget_15")
        self.horizontalLayout_22 = QHBoxLayout(self.widget_15)
        self.horizontalLayout_22.setObjectName(u"horizontalLayout_22")
        self.horizontalLayout_22.setContentsMargins(1, 1, 1, 1)
        self.label_2 = QLabel(self.widget_15)
        self.label_2.setObjectName(u"label_2")

        self.horizontalLayout_22.addWidget(self.label_2)

        self.pp_playlist_name = QLineEdit(self.widget_15)
        self.pp_playlist_name.setObjectName(u"pp_playlist_name")
        self.pp_playlist_name.setAlignment(Qt.AlignRight|Qt.AlignTrailing|Qt.AlignVCenter)

        self.horizontalLayout_22.addWidget(self.pp_playlist_name)


        self.verticalLayout_15.addWidget(self.widget_15)

        self.pp_save_playlist = QPushButton(self.groupBox_4)
        self.pp_save_playlist.setObjectName(u"pp_save_playlist")

        self.verticalLayout_15.addWidget(self.pp_save_playlist)

        self.pp_new_playlist = QPushButton(self.groupBox_4)
        self.pp_new_playlist.setObjectName(u"pp_new_playlist")

        self.verticalLayout_15.addWidget(self.pp_new_playlist)

        self.pp_delete_playlist = QPushButton(self.groupBox_4)
        self.pp_delete_playlist.setObjectName(u"pp_delete_playlist")

        self.verticalLayout_15.addWidget(self.pp_delete_playlist)

        self.pp_load_playlist = QPushButton(self.groupBox_4)
        self.pp_load_playlist.setObjectName(u"pp_load_playlist")

        self.verticalLayout_15.addWidget(self.pp_load_playlist)

        self.pp_export_playlist = QPushButton(self.groupBox_4)
        self.pp_export_playlist.setObjectName(u"pp_export_playlist")

        self.verticalLayout_15.addWidget(self.pp_export_playlist)

        self.splitter_4.addWidget(self.groupBox_4)

        self.horizontalLayout_16.addWidget(self.splitter_4)

        self.tabWidget1.addTab(self.tab_7, "")
        self.splitter.addWidget(self.tabWidget1)

        self.verticalLayout.addWidget(self.splitter)

        self.tabWidget.addTab(self.tab_1, "")
        self.tab_5 = QWidget()
        self.tab_5.setObjectName(u"tab_5")
        self.horizontalLayout_30 = QHBoxLayout(self.tab_5)
        self.horizontalLayout_30.setObjectName(u"horizontalLayout_30")
        self.splitter_13 = QSplitter(self.tab_5)
        self.splitter_13.setObjectName(u"splitter_13")
        self.splitter_13.setOrientation(Qt.Horizontal)
        self.splitter_11 = QSplitter(self.splitter_13)
        self.splitter_11.setObjectName(u"splitter_11")
        self.splitter_11.setOrientation(Qt.Vertical)
        self.widget_24 = QWidget(self.splitter_11)
        self.widget_24.setObjectName(u"widget_24")
        self.verticalLayout_21 = QVBoxLayout(self.widget_24)
        self.verticalLayout_21.setObjectName(u"verticalLayout_21")
        self.widget_19 = QWidget(self.widget_24)
        self.widget_19.setObjectName(u"widget_19")
        self.horizontalLayout_25 = QHBoxLayout(self.widget_19)
        self.horizontalLayout_25.setObjectName(u"horizontalLayout_25")
        self.label_8 = QLabel(self.widget_19)
        self.label_8.setObjectName(u"label_8")

        self.horizontalLayout_25.addWidget(self.label_8)

        self.fm_folder_format = QLineEdit(self.widget_19)
        self.fm_folder_format.setObjectName(u"fm_folder_format")

        self.horizontalLayout_25.addWidget(self.fm_folder_format)


        self.verticalLayout_21.addWidget(self.widget_19)

        self.widget_22 = QWidget(self.widget_24)
        self.widget_22.setObjectName(u"widget_22")
        self.horizontalLayout_27 = QHBoxLayout(self.widget_22)
        self.horizontalLayout_27.setObjectName(u"horizontalLayout_27")
        self.label_10 = QLabel(self.widget_22)
        self.label_10.setObjectName(u"label_10")

        self.horizontalLayout_27.addWidget(self.label_10)

        self.fm_song_name_format = QLineEdit(self.widget_22)
        self.fm_song_name_format.setObjectName(u"fm_song_name_format")

        self.horizontalLayout_27.addWidget(self.fm_song_name_format)


        self.verticalLayout_21.addWidget(self.widget_22)

        self.splitter_11.addWidget(self.widget_24)
        self.widget_28 = QWidget(self.splitter_11)
        self.widget_28.setObjectName(u"widget_28")
        self.verticalLayout_23 = QVBoxLayout(self.widget_28)
        self.verticalLayout_23.setObjectName(u"verticalLayout_23")
        self.widget_29 = QWidget(self.widget_28)
        self.widget_29.setObjectName(u"widget_29")
        self.horizontalLayout_29 = QHBoxLayout(self.widget_29)
        self.horizontalLayout_29.setObjectName(u"horizontalLayout_29")
        self.horizontalLayout_29.setContentsMargins(9, 9, 9, 9)
        self.label_13 = QLabel(self.widget_29)
        self.label_13.setObjectName(u"label_13")

        self.horizontalLayout_29.addWidget(self.label_13)

        self.fm_selective_scan_folder = QLineEdit(self.widget_29)
        self.fm_selective_scan_folder.setObjectName(u"fm_selective_scan_folder")

        self.horizontalLayout_29.addWidget(self.fm_selective_scan_folder)

        self.fm_update = QCheckBox(self.widget_29)
        self.fm_update.setObjectName(u"fm_update")
        self.fm_update.setLayoutDirection(Qt.RightToLeft)
        self.fm_update.setChecked(True)
        self.fm_update.setTristate(False)

        self.horizontalLayout_29.addWidget(self.fm_update)


        self.verticalLayout_23.addWidget(self.widget_29)

        self.widget_32 = QWidget(self.widget_28)
        self.widget_32.setObjectName(u"widget_32")
        self.verticalLayout_26 = QVBoxLayout(self.widget_32)
        self.verticalLayout_26.setObjectName(u"verticalLayout_26")
        self.fm_selective_scan = QPushButton(self.widget_32)
        self.fm_selective_scan.setObjectName(u"fm_selective_scan")

        self.verticalLayout_26.addWidget(self.fm_selective_scan)

        self.fm_find_moved_files = QPushButton(self.widget_32)
        self.fm_find_moved_files.setObjectName(u"fm_find_moved_files")

        self.verticalLayout_26.addWidget(self.fm_find_moved_files)

        self.fm_commit_datebase = QPushButton(self.widget_32)
        self.fm_commit_datebase.setObjectName(u"fm_commit_datebase")

        self.verticalLayout_26.addWidget(self.fm_commit_datebase)


        self.verticalLayout_23.addWidget(self.widget_32)

        self.splitter_11.addWidget(self.widget_28)
        self.widget_25 = QWidget(self.splitter_11)
        self.widget_25.setObjectName(u"widget_25")
        self.verticalLayout_19 = QVBoxLayout(self.widget_25)
        self.verticalLayout_19.setObjectName(u"verticalLayout_19")
        self.widget_18 = QWidget(self.widget_25)
        self.widget_18.setObjectName(u"widget_18")
        self.horizontalLayout_24 = QHBoxLayout(self.widget_18)
        self.horizontalLayout_24.setObjectName(u"horizontalLayout_24")
        self.horizontalLayout_24.setContentsMargins(9, 9, 9, 9)
        self.label_7 = QLabel(self.widget_18)
        self.label_7.setObjectName(u"label_7")

        self.horizontalLayout_24.addWidget(self.label_7)

        self.fm_scan_and_delete_duplicate_folder = QLineEdit(self.widget_18)
        self.fm_scan_and_delete_duplicate_folder.setObjectName(u"fm_scan_and_delete_duplicate_folder")

        self.horizontalLayout_24.addWidget(self.fm_scan_and_delete_duplicate_folder)


        self.verticalLayout_19.addWidget(self.widget_18)

        self.fm_scan_and_delete_duplicate = QPushButton(self.widget_25)
        self.fm_scan_and_delete_duplicate.setObjectName(u"fm_scan_and_delete_duplicate")

        self.verticalLayout_19.addWidget(self.fm_scan_and_delete_duplicate)

        self.splitter_11.addWidget(self.widget_25)
        self.widget_31 = QWidget(self.splitter_11)
        self.widget_31.setObjectName(u"widget_31")
        self.verticalLayout_25 = QVBoxLayout(self.widget_31)
        self.verticalLayout_25.setObjectName(u"verticalLayout_25")
        self.widget_30 = QWidget(self.widget_31)
        self.widget_30.setObjectName(u"widget_30")
        self.verticalLayout_24 = QVBoxLayout(self.widget_30)
        self.verticalLayout_24.setObjectName(u"verticalLayout_24")

        self.verticalLayout_25.addWidget(self.widget_30)

        self.widget_26 = QWidget(self.widget_31)
        self.widget_26.setObjectName(u"widget_26")
        self.verticalLayout_22 = QVBoxLayout(self.widget_26)
        self.verticalLayout_22.setObjectName(u"verticalLayout_22")
        self.widget_20 = QWidget(self.widget_26)
        self.widget_20.setObjectName(u"widget_20")
        self.verticalLayout_18 = QVBoxLayout(self.widget_20)
        self.verticalLayout_18.setObjectName(u"verticalLayout_18")
        self.widget_21 = QWidget(self.widget_20)
        self.widget_21.setObjectName(u"widget_21")
        self.horizontalLayout_26 = QHBoxLayout(self.widget_21)
        self.horizontalLayout_26.setObjectName(u"horizontalLayout_26")
        self.label_9 = QLabel(self.widget_21)
        self.label_9.setObjectName(u"label_9")

        self.horizontalLayout_26.addWidget(self.label_9)

        self.fm_rearrange_files_folder = QLineEdit(self.widget_21)
        self.fm_rearrange_files_folder.setObjectName(u"fm_rearrange_files_folder")

        self.horizontalLayout_26.addWidget(self.fm_rearrange_files_folder)


        self.verticalLayout_18.addWidget(self.widget_21)

        self.widget_23 = QWidget(self.widget_20)
        self.widget_23.setObjectName(u"widget_23")
        self.horizontalLayout_28 = QHBoxLayout(self.widget_23)
        self.horizontalLayout_28.setObjectName(u"horizontalLayout_28")
        self.label_11 = QLabel(self.widget_23)
        self.label_11.setObjectName(u"label_11")

        self.horizontalLayout_28.addWidget(self.label_11)

        self.fm_rearrange_format = QLineEdit(self.widget_23)
        self.fm_rearrange_format.setObjectName(u"fm_rearrange_format")

        self.horizontalLayout_28.addWidget(self.fm_rearrange_format)


        self.verticalLayout_18.addWidget(self.widget_23)

        self.widget_27 = QWidget(self.widget_20)
        self.widget_27.setObjectName(u"widget_27")
        self.horizontalLayout_2 = QHBoxLayout(self.widget_27)
        self.horizontalLayout_2.setObjectName(u"horizontalLayout_2")
        self.label_12 = QLabel(self.widget_27)
        self.label_12.setObjectName(u"label_12")

        self.horizontalLayout_2.addWidget(self.label_12)

        self.fm_origin_format = QLineEdit(self.widget_27)
        self.fm_origin_format.setObjectName(u"fm_origin_format")

        self.horizontalLayout_2.addWidget(self.fm_origin_format)


        self.verticalLayout_18.addWidget(self.widget_27)

        self.fm_rearrange_files = QPushButton(self.widget_20)
        self.fm_rearrange_files.setObjectName(u"fm_rearrange_files")

        self.verticalLayout_18.addWidget(self.fm_rearrange_files)


        self.verticalLayout_22.addWidget(self.widget_20)

        self.verticalLayout_22.setStretch(0, 3)

        self.verticalLayout_25.addWidget(self.widget_26)

        self.splitter_11.addWidget(self.widget_31)
        self.splitter_13.addWidget(self.splitter_11)
        self.splitter_12 = QSplitter(self.splitter_13)
        self.splitter_12.setObjectName(u"splitter_12")
        self.splitter_12.setOrientation(Qt.Vertical)
        self.plainTextEdit_2 = QPlainTextEdit(self.splitter_12)
        self.plainTextEdit_2.setObjectName(u"plainTextEdit_2")
        self.splitter_12.addWidget(self.plainTextEdit_2)
        self.plainTextEdit = QPlainTextEdit(self.splitter_12)
        self.plainTextEdit.setObjectName(u"plainTextEdit")
        self.plainTextEdit.setReadOnly(True)
        self.splitter_12.addWidget(self.plainTextEdit)
        self.textEdit = QTextEdit(self.splitter_12)
        self.textEdit.setObjectName(u"textEdit")
        self.textEdit.setReadOnly(True)
        self.splitter_12.addWidget(self.textEdit)
        self.splitter_13.addWidget(self.splitter_12)

        self.horizontalLayout_30.addWidget(self.splitter_13)

        self.tabWidget.addTab(self.tab_5, "")

        self.verticalLayout_20.addWidget(self.tabWidget)

        mainWindow.setCentralWidget(self.centralwidget)
        self.menubar = QMenuBar(mainWindow)
        self.menubar.setObjectName(u"menubar")
        self.menubar.setGeometry(QRect(0, 0, 1067, 22))
        self.menubar.setDefaultUp(False)
        self.menuFile = QMenu(self.menubar)
        self.menuFile.setObjectName(u"menuFile")
        self.menuRefresh = QMenu(self.menubar)
        self.menuRefresh.setObjectName(u"menuRefresh")
        mainWindow.setMenuBar(self.menubar)
        self.statusbar = QStatusBar(mainWindow)
        self.statusbar.setObjectName(u"statusbar")
        mainWindow.setStatusBar(self.statusbar)

        self.menubar.addAction(self.menuFile.menuAction())
        self.menubar.addAction(self.menuRefresh.menuAction())
        self.menuFile.addAction(self.actionScanFileDatabasePath)
        self.menuFile.addAction(self.actionBackupDatabase)
        self.menuFile.addAction(self.actionCleanDatabase)
        self.menuFile.addAction(self.actionCleanSongSingerAuthor)
        self.menuFile.addAction(self.actionSetting)
        self.menuRefresh.addAction(self.actionRestartWindow)
        self.menuRefresh.addAction(self.actionResetPPD)
        self.menuRefresh.addAction(self.actionResetPlaylist)

        self.retranslateUi(mainWindow)

        self.tabWidget.setCurrentIndex(0)
        self.tabWidget_3.setCurrentIndex(0)
        self.tabWidget_10.setCurrentIndex(0)
        self.tabWidget1.setCurrentIndex(0)


        QMetaObject.connectSlotsByName(mainWindow)
    # setupUi

    def retranslateUi(self, mainWindow):
        mainWindow.setWindowTitle(QCoreApplication.translate("mainWindow", u"VmdManger", None))
        self.actionScanFileDatabasePath.setText(QCoreApplication.translate("mainWindow", u"ScanFileDatabasePath", None))
        self.actionBackupDatabase.setText(QCoreApplication.translate("mainWindow", u"BackupDatabase", None))
        self.actionSetting.setText(QCoreApplication.translate("mainWindow", u"Setting", None))
        self.actionRestartWindow.setText(QCoreApplication.translate("mainWindow", u"RestartWindow", None))
        self.actionResetPPD.setText(QCoreApplication.translate("mainWindow", u"ResetPPD", None))
        self.actionResetPlaylist.setText(QCoreApplication.translate("mainWindow", u"ResetPlaylist", None))
        self.actionCleanDatabase.setText(QCoreApplication.translate("mainWindow", u"CleanDatabase", None))
        self.actionCleanSongSingerAuthor.setText(QCoreApplication.translate("mainWindow", u"CleanSongSingerAuthor", None))
        self.groupBox_3.setTitle(QCoreApplication.translate("mainWindow", u"PerformancePlan", None))
        self.pp_performance_plan_search.setInputMask("")
        self.pp_performance_plan_search.setPlaceholderText(QCoreApplication.translate("mainWindow", u"\u641c\u7d22\uff1a\u57fa\u4e8eplan name", None))
        self.pp_song_search.setPlaceholderText(QCoreApplication.translate("mainWindow", u"\u641c\u7d22", None))
        self.tabWidget_3.setTabText(self.tabWidget_3.indexOf(self.tab_9), QCoreApplication.translate("mainWindow", u"Song", None))
        self.pp_author_search.setPlaceholderText(QCoreApplication.translate("mainWindow", u"\u641c\u7d22", None))
        self.tabWidget_3.setTabText(self.tabWidget_3.indexOf(self.tab_10), QCoreApplication.translate("mainWindow", u"Author", None))
        self.pp_tag_search.setPlaceholderText(QCoreApplication.translate("mainWindow", u"\u641c\u7d22", None))
        self.pp_delete_tag.setText(QCoreApplication.translate("mainWindow", u"DeleteTag", None))
        self.tabWidget_3.setTabText(self.tabWidget_3.indexOf(self.tab_11), QCoreApplication.translate("mainWindow", u"Tag", None))
        self.pp_filter.setPlaceholderText(QCoreApplication.translate("mainWindow", u"Filter: tag\u53ea\u5bf9plan\u6709\u6548", None))
        self.groupBox_2.setTitle(QCoreApplication.translate("mainWindow", u"PerformancePlanDetail", None))
        self.label_5.setText(QCoreApplication.translate("mainWindow", u"PlanId", None))
        self.pp_ppd_plan_id.setText(QCoreApplication.translate("mainWindow", u"0", None))
        self.label.setText(QCoreApplication.translate("mainWindow", u"PlanName", None))
        self.label_3.setText(QCoreApplication.translate("mainWindow", u"DancerNumber", None))
        self.groupBox_11.setTitle(QCoreApplication.translate("mainWindow", u"Dancer", None))
        self.pp_save_plan.setText(QCoreApplication.translate("mainWindow", u"SavePlan", None))
        self.pp_new_plan.setText(QCoreApplication.translate("mainWindow", u"NewPlan", None))
        self.pp_delete_plan.setText(QCoreApplication.translate("mainWindow", u"DeletePlan", None))
        self.groupBox_12.setTitle(QCoreApplication.translate("mainWindow", u"Camera", None))
        self.groupBox_13.setTitle(QCoreApplication.translate("mainWindow", u"Music", None))
        self.tabWidget_10.setTabText(self.tabWidget_10.indexOf(self.tabWidget_10Page1), QCoreApplication.translate("mainWindow", u"CM", None))
        self.label_4.setText(QCoreApplication.translate("mainWindow", u"Rating\uff080\u4e3a\u4e0d\u6253\u5206\uff09", None))
        self.pp_ppd_rating.setSuffix("")
        self.groupBox_14.setTitle(QCoreApplication.translate("mainWindow", u"tags", None))
        self.pp_ppd_add_tag.setText(QCoreApplication.translate("mainWindow", u"Add", None))
        self.tabWidget_10.setTabText(self.tabWidget_10.indexOf(self.tabWidget_10Page2), QCoreApplication.translate("mainWindow", u"RT", None))
        self.pp_ppd_comments.setHtml(QCoreApplication.translate("mainWindow", u"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \"http://www.w3.org/TR/REC-html40/strict.dtd\">\n"
"<html><head><meta name=\"qrichtext\" content=\"1\" /><meta charset=\"utf-8\" /><style type=\"text/css\">\n"
"p, li { white-space: pre-wrap; }\n"
"hr { height: 1px; border-width: 0; }\n"
"li.unchecked::marker { content: \"\\2610\"; }\n"
"li.checked::marker { content: \"\\2612\"; }\n"
"</style></head><body style=\" font-family:'Microsoft YaHei UI'; font-size:9pt; font-weight:400; font-style:normal;\">\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\">\u65e0\u8bc4\u8bba</p></body></html>", None))
        self.tabWidget_10.setTabText(self.tabWidget_10.indexOf(self.tab_3), QCoreApplication.translate("mainWindow", u"Comments", None))
        self.groupBox_6.setTitle(QCoreApplication.translate("mainWindow", u"Dancer", None))
        self.groupBox_7.setTitle(QCoreApplication.translate("mainWindow", u"Camera", None))
        self.groupBox_8.setTitle(QCoreApplication.translate("mainWindow", u"DancerMotion", None))
        self.groupBox_9.setTitle(QCoreApplication.translate("mainWindow", u"DancerMorph", None))
        self.groupBox_10.setTitle(QCoreApplication.translate("mainWindow", u"Music", None))
        self.tabWidget1.setTabText(self.tabWidget1.indexOf(self.tabWidgetPage1), QCoreApplication.translate("mainWindow", u"File", None))
        self.groupBox.setTitle(QCoreApplication.translate("mainWindow", u"Plan in Playlist", None))
        self.pp_add_filtered_plan.setText(QCoreApplication.translate("mainWindow", u"AddFilteredPlan", None))
        self.pp_remove_duplicate_entries.setText(QCoreApplication.translate("mainWindow", u"RemoveDuplicateDntries", None))
        self.groupBox_4.setTitle(QCoreApplication.translate("mainWindow", u"Playlist", None))
        self.pp_playlist_search.setPlaceholderText(QCoreApplication.translate("mainWindow", u"\u641c\u7d22", None))
        self.label_6.setText(QCoreApplication.translate("mainWindow", u"PlaylistID", None))
        self.pp_playlist_id.setText(QCoreApplication.translate("mainWindow", u"0", None))
        self.label_2.setText(QCoreApplication.translate("mainWindow", u"PlaylistName", None))
        self.pp_save_playlist.setText(QCoreApplication.translate("mainWindow", u"SavePlaylist", None))
        self.pp_new_playlist.setText(QCoreApplication.translate("mainWindow", u"NewPlaylist", None))
        self.pp_delete_playlist.setText(QCoreApplication.translate("mainWindow", u"DeletePlaylist", None))
        self.pp_load_playlist.setText(QCoreApplication.translate("mainWindow", u"LoadPlaylist : Ensure files scanned", None))
        self.pp_export_playlist.setText(QCoreApplication.translate("mainWindow", u"ExportPlaylist", None))
        self.tabWidget1.setTabText(self.tabWidget1.indexOf(self.tab_7), QCoreApplication.translate("mainWindow", u"Playlist", None))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_1), QCoreApplication.translate("mainWindow", u"PerformancePlan", None))
        self.label_8.setText(QCoreApplication.translate("mainWindow", u"FolderFormat", None))
        self.label_10.setText(QCoreApplication.translate("mainWindow", u"SongNameFormat", None))
        self.label_13.setText(QCoreApplication.translate("mainWindow", u"SelectiveScan", None))
        self.fm_update.setText(QCoreApplication.translate("mainWindow", u"Update", None))
        self.fm_selective_scan.setText(QCoreApplication.translate("mainWindow", u"SelectiveScan", None))
        self.fm_find_moved_files.setText(QCoreApplication.translate("mainWindow", u"FindMovedFiles", None))
        self.fm_commit_datebase.setText(QCoreApplication.translate("mainWindow", u"CommitDatebase", None))
        self.label_7.setText(QCoreApplication.translate("mainWindow", u"ScanAndDelete", None))
        self.fm_scan_and_delete_duplicate.setText(QCoreApplication.translate("mainWindow", u"ScanAndDeleteDuplicate", None))
        self.label_9.setText(QCoreApplication.translate("mainWindow", u"RearrangeFolder", None))
        self.label_11.setText(QCoreApplication.translate("mainWindow", u"RearrangeFormat", None))
        self.label_12.setText(QCoreApplication.translate("mainWindow", u"OriginFormat", None))
        self.fm_rearrange_files.setText(QCoreApplication.translate("mainWindow", u"RearrangeFile", None))
        self.plainTextEdit.setPlainText(QCoreApplication.translate("mainWindow", u"\u6240\u6709\u7684\u8def\u5f84\u90fd\u9700\u8981\u662f\u7edd\u5bf9\u8def\u5f84\n"
"\u5355\u7eaf\u7684\u626b\u63cf\u4e0d\u4f1a\u63d0\u4ea4\u6570\u636e\u5e93\u6539\u52a8\uff0c\u70b9\u51fbcommit\u624d\u4f1a\n"
"\u626b\u63cf\u6587\u4ef6\u6570\u636e\u5e93\u4ee5\u5916\u7684\u4f4d\u7f6e\uff0c\u4f1a\u81ea\u52a8\u79fb\u52a8\u4e0e\u6570\u636e\u5e93\u5185\u5df2\u6709\u7684\u6587\u4ef6\u76f8\u540c\u7684\u6587\u4ef6\u5230duplicate\u6587\u4ef6\u5939\u5185\n"
"", None))
        self.textEdit.setHtml(QCoreApplication.translate("mainWindow", u"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \"http://www.w3.org/TR/REC-html40/strict.dtd\">\n"
"<html><head><meta name=\"qrichtext\" content=\"1\" /><meta charset=\"utf-8\" /><style type=\"text/css\">\n"
"p, li { white-space: pre-wrap; }\n"
"hr { height: 1px; border-width: 0; }\n"
"li.unchecked::marker { content: \"\\2610\"; }\n"
"li.checked::marker { content: \"\\2612\"; }\n"
"</style></head><body style=\" font-family:'Microsoft YaHei UI'; font-size:9pt; font-weight:400; font-style:normal;\">\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">\u5f62\u5982author-song-...</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">\u4ece\u524d\u5f80\u540e\uff0c\u4f18\u5148\u8bfb\u53d6\u786e\u8ba4\u7684\u5b57\u6bb5</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; "
                        "margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">\u53ef\u80fd\u6709\u7684\u5b57\u6bb5\uff1a</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">'author': author.name,</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">'song': song.name,</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">'singer': singer.name,</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">'date': file.date</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;"
                        "\"><span style=\" font-size:12pt;\">'tag': tag.name</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">'...':\u4e2d\u95f4\u6216\u6700\u540e\u7684\u503c\uff0c\u53ea\u80fd\u51fa\u73b0\u4e00\u6b21\uff0c\u4e0d\u5199\u4f1a\u4e22\u5931\u539f\u6587\u4ef6\u7684\u591a\u5c42\u7ea7\u7ed3\u6784\u3002\u91cd\u6574\u4e2d\u53ea\u6709\u8fd9\u4e2a\u503c\u7684\u4f4d\u7f6e\u662f\u91cd\u8981\u7684\uff0c\u5176\u4ed6\u503c\u4f1a\u76f4\u63a5\u4ece\u6570\u636e\u5e93\u4e2d\u83b7\u53d6</span></p>\n"
"<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:12pt;\">\u5176\u4ed6\u5b57\u6bb5\u4f1a\u4f5c\u4e3a\u5360\u4f4d\u7b26\u5b58\u5728\uff0c\u4e0d\u4f1a\u88ab\u8bfb\u53d6\uff0c\u5982\u679cFolderFormat\u4e2d\u51fa\u73b0\u4e86singer\uff0c\u5219SongNameFormat\u4f1a\u88ab\u5ffd\u7565</span></p></body></html>", None))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_5), QCoreApplication.translate("mainWindow", u"FileManger", None))
        self.menuFile.setTitle(QCoreApplication.translate("mainWindow", u"File", None))
        self.menuRefresh.setTitle(QCoreApplication.translate("mainWindow", u"Refresh", None))
    # retranslateUi


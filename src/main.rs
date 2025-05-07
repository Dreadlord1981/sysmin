use anyhow::anyhow;
use clap::Parser;
use sysmin::{api::{CssOperation, Operation}, cli::Arguments, Executor};
fn main() -> Result<(), anyhow::Error> {
    
	let args = Arguments::parse();

	if !args.icecap && !args.portfolio {
		return Err(anyhow!("Need to choose a package to build, try --help to see help info"));
	}

	if args.icecap {

		let mut files: Vec<&str> = args.list.split_whitespace().collect();
		let mut extra: Vec<&str> = args.extra.split_whitespace().collect();

		if files.is_empty() {

			files = vec![
				"$POPATH/../icecap/app/ux/form/panel/Panel4Area.js",
				"$POPATH/../icecap/app/ux/form/panel/TabPanel.js",
				"$POPATH/../icecap/app/ux/grid/editor/Controller.js",
				"$POPATH/../icecap/app/ux/grid/editor/Main.js",
				"$POPATH/../icecap/app/ux/container/Column.js",
				"$POPATH/../icecap/app/ux/button/Button.js",
				"$POPATH/../icecap/app/ux/form/label/Label.js",
				"$POPATH/../icecap/app/ux/form/field/Field.js",
				"$POPATH/../icecap/app/ux/form/panel/fusionChart.js",
				"$POPATH/../icecap/app/ux/form/field/SimpleComboBox.js",
				"$POPATH/../icecap/app/ux/form/field/wdsf/PushButtons.js",
				"$POPATH/../icecap/app/ux/form/field/wdsf/CheckBoxGroup.js",
				"$POPATH/../icecap/app/ux/form/field/wdsf/RadioGroup.js",
				"$POPATH/../icecap/app/ux/form/field/sqlComboBox.js",
				"$POPATH/../icecap/app/ux/form/field/sqlSearchBox.js",
				"$POPATH/../icecap/app/ux/form/field/LINKFIELD.js",
				"$POPATH/../icecap/app/ux/form/field/AdvancedComboBox.js",
				"$POPATH/../icecap/app/ux/form/field/CommandField.js",
				"$POPATH/../icecap/app/ux/form/field/LibraryFileDependency.js",
				"$POPATH/../icecap/app/ux/form/field/FileLibraryDependency.js",
				"$POPATH/../icecap/app/ux/form/field/RadioGroup.js",
				"$POPATH/../icecap/app/ux/form/field/i18n/TextField.js",
				"$POPATH/../icecap/app/ux/form/field/ActionField.js",
				"$POPATH/../icecap/app/ux/form/field/MailTo.js",
				"$POPATH/../icecap/app/ux/form/field/WebSearch.js",
				"$POPATH/../icecap/app/ux/form/field/Map.js",
				"$POPATH/../icecap/app/ux/form/field/Function.js",
				"$POPATH/../icecap/app/ux/container/CommandPanel.js",
				"$POPATH/../icecap/app/ux/container/GMapPanel.js",
				"$POPATH/../icecap/app/ux/map/Panel.js",
				"$POPATH/../icecap/app/ux/panel/controller/Simple.js",
				"$POPATH/../icecap/app/ux/panel/view/Simple.js",
				"$POPATH/../icecap/app/ux/panel/controller/Flex.js",
				"$POPATH/../icecap/app/ux/panel/view/Flex.js"
			]
		}

		if extra.is_empty() {

			extra = vec![
				"$POPATH/../icecap/app/component/Marker.js",
				"$POPATH/../icecap/app/component/Cursor.js",
				"$POPATH/../icecap/app/component/Input.js",
				"$POPATH/../icecap/app/component/Selector.js",
				"$POPATH/../icecap/app/view/WebContainer.js",
				"$POPATH/../icecap/app/util/Watchdog.js",
				"$POPATH/../icecap/app/util/Scale.js",
				"$POPATH/../icecap/app/util/Breakout.js",
				"$POPATH/../icecap/app/controller/Window.js",
				"$POPATH/../icecap/app/view/Window.js",
				"$POPATH/../icecap/app/controller/Main.js",
				"$POPATH/../icecap/app/view/Main.js"
			]
		}

		let mut op = Operation::new("Icecap - ux", "$POPATH/../icecap/app/ux.js", "$POPATH/../icecap/app/ux-debug.js");
		op.files = files;
		op.run()?;

		op = Operation::new("Icecap - system", "$POPATH/../icecap/app/system.js", "$POPATH/../icecap/app/system-debug.js");
		op.files = extra;
		op.run()?
	}
	else if args.portfolio {

		let mut files: Vec<&str> = args.list.split_whitespace().collect();

		if files.is_empty() {

			files = vec![
				"$POPATH/app/util/Theme.js",
				"$POPATH/app/state/local/Provider.js",
				"$POPATH/app/Router.js",
				"$POPATH/app/util/jquery-1.9.0.min.js",				
				"$POPATH/app/util/raphael.2.1.0.min.js",
				"$POPATH/app/ux/ckeditor/ckeditor.js",
				"$POPATH/ext-override.js",
				"$POPATH/app/util/Handler.js",
				"$POPATH/app/state/global/Provider.js",
				"$POPATH/app/state/profile/Provider.js",
				"$POPATH/app/defaults/uitype.js",
				"$POPATH/app/ux/Badge.js",
				"$POPATH/app/ux/form/field/CKEditorField.js",
				"$POPATH/app/ux/container/Widget.js",
				"$POPATH/app/ux/plugin/data/Tracker.js",
				"$POPATH/app/ux/container/Ip2Widget.js",
				"$POPATH/app/notification/view/Editor.js",
				"$POPATH/app/notification/util/Socket.js",
				"$POPATH/app/notification/view/Main.js",
				"$POPATH/app/message/model/Error.js",
				"$POPATH/app/message/view/Main.js",
				"$POPATH/app/util/Message.js",
				"$POPATH/app/login/util/Handler.js",
				"$POPATH/app/ux/BoxReorderer.js",
				"$POPATH/app/ux/TabReorderer.js",
				"$POPATH/app/ux/TabCloseMenu.js",
				"$POPATH/app/center/view/Main.js",
				"$POPATH/app/menu/model/System.js",
				"$POPATH/app/menu/view/Favorite.js",
				"$POPATH/app/menu/view/Main.js",
				"$POPATH/app/top/view/Main.js",
				"$POPATH/app/view/Main.js",
				"$POPATH/app/helper/view/Main.js",
				"$POPATH/app/ux/form/field/Search.js",
				"$POPATH/app/menu/view/System.js",
				"$POPATH/app/notification/util/Notification.js",
				"$POPATH/app/util/jquery.utilities.js"
				
			];
		}

		let mut op = Operation::new("Portfolio - boot", "$POPATH/system-boot.js", "$POPATH/system-boot-debug.js");
		op.files = files;
		op.extra = vec![];
		op.append = true;
		op.run()?;
		
		let mut extra: Vec<&str> = args.extra.split_whitespace().collect();

		let mut files = vec![
			"$POPATH/app/data/field/Json.js",
			"$POPATH/app/defaults/uirules.js",
			"$POPATH/app/dynapp/util/Service.js",
			"$POPATH/app/registry/util/Service.js",
			"$POPATH/app/util/Mammoth.js",
			"$POPATH/app/util/papaparse.min.js",
			"$POPATH/app/util/xlsx.full.min.js",
			"$POPATH/app/ux/simplemde/dist/simplemde.min.js",
			"$POPATH/app/util/Xpd.js",
			"$POPATH/app/util/Wsh.js",
			"$POPATH/app/util/EmlParser.js",
			"$POPATH/app/ux/Notification.js",
			"$POPATH/app/ux/InfoPanel.js",
			"$POPATH/app/ux/InfoPanelQueue.js",
			"$POPATH/app/ux/util/ComponentController.js",
			"$POPATH/app/ux/util/KeyMapping.js",
			"$POPATH/app/ux/plugin/tree/NodeDisabled.js",
			"$POPATH/app/ux/plugin/DragDrop.js",
			"$POPATH/app/angular/Component.js",
			"$POPATH/app/ux/form/JsonForm.js",
			"$POPATH/app/ux/plugin/UploadDD.js",
			"$POPATH/app/ux/form/field/plugin/InputMask.js",
			"$POPATH/app/ux/form/field/plugin/SuffixLabel.js",
			"$POPATH/app/ux/form/field/plugin/Unique.js",
			"$POPATH/app/ux/form/field/QuickCombo.js",
			"$POPATH/app/ux/form/field/AutoTimeStamp.js",
			"$POPATH/app/ux/themed/Button.js",
			"$POPATH/app/ux/themed/MenuItem.js",
			"$POPATH/app/ux/themed/SplitButton.js",
			"$POPATH/app/ux/themed/ActionColumn.js",
			"$POPATH/app/ux/themed/FAB.js",
			"$POPATH/app/ux/xpd/I18nField.js",
			"$POPATH/app/ux/call2action/GeneralPanel.js",
			"$POPATH/app/ux/call2action/UrlPanel.js",
			"$POPATH/app/ux/call2action/PagingToolbar.js",
			"$POPATH/app/ux/call2action/EntityGrid.js",
			"$POPATH/app/ux/call2action/SitePanel.js",
			"$POPATH/app/ux/container/Iframe.js",
			"$POPATH/app/ux/container/angular/Iframe.js",
			"$POPATH/app/ux/container/Icecap.js",
			"$POPATH/app/ux/data/PagingMemoryProxy.js",
			"$POPATH/app/ux/folderorganizer/Alert.js",
			"$POPATH/app/ux/folderorganizer/Button.js",
			"$POPATH/app/ux/folderorganizer/ItemRelationSelector.js",
			"$POPATH/app/ux/folderorganizer/MultiChoice.js",
			"$POPATH/app/ux/folderorganizer/PagingToolbar.js",
			"$POPATH/app/ux/folderorganizer/PropertyGrid.js",
			"$POPATH/app/ux/folderorganizer/FolderPropertyGrid.js",
			"$POPATH/app/ux/folderorganizer/FolderProductLocation.js",
			"$POPATH/app/ux/folderorganizer/SearchField.js",
			"$POPATH/app/ux/folderorganizer/Toolbar.js",
			"$POPATH/app/ux/folderorganizer/TreeSelModel.js",
			"$POPATH/app/ux/folderorganizer/Util.js",
			"$POPATH/app/ux/folderorganizer/Itemgrid.js",
			"$POPATH/app/ux/folderorganizer/Content.js",
			"$POPATH/app/ux/folderorganizer/TreeSelector.js",
			"$POPATH/app/ux/folderorganizer/Selector.js",
			"$POPATH/app/ux/mailChimp/Panel.js",
			"$POPATH/app/ux/mailChimp/Grid.js",
			"$POPATH/app/ux/mailChimp/ListCombo.js",
			"$POPATH/app/ux/mailChimp/SearchPanel.js",
			"$POPATH/app/ux/data/model/Locale.js",
			"$POPATH/app/ux/data/store/Locale.js",
			"$POPATH/app/ux/form/field/LocaleComboBox.js",
			"$POPATH/app/ux/form/field/BasketItems.js",
			"$POPATH/app/ux/form/field/GridPicker.js",
			"$POPATH/app/ux/form/field/UploadPicker.js",
			"$POPATH/app/ux/form/field/DownloadPicker.js",
			"$POPATH/app/ux/form/field/IncludeComboBox.js",
			"$POPATH/app/ux/form/field/IsoTimestamp.js",
			"$POPATH/app/ux/form/field/ISOTimestampDisplayField.js",
			"$POPATH/app/ux/form/field/ISOTimestampField.js",
			"$POPATH/app/ux/form/field/ISOTimestampPreSaveField.js",
			"$POPATH/app/ux/form/field/KeywordDensityChecker.js",
			"$POPATH/app/ux/form/field/ThumbnailComboBox.js",
			"$POPATH/app/ux/form/ExtendedFieldContainer.js",
			"$POPATH/app/ux/form/field/i18n/text/TextArea.js",
			"$POPATH/app/ux/form/field/i18n/text/TextField.js",
			"$POPATH/app/ux/form/field/i18n/text/MarkdownField.js",
			"$POPATH/app/ux/form/field/i18n/text/UploadPickerField.js",
			"$POPATH/app/ux/form/field/i18n/text/CKEditorField.js",
			"$POPATH/app/ux/form/field/i18n/hook/TextField.js",
			"$POPATH/app/ux/form/field/i18n/hook/TextArea.js",
			"$POPATH/app/ux/form/field/pd/Picker.js",
			"$POPATH/app/ux/form/field/pd/tree/Picker.js",
			"$POPATH/app/ux/form/field/BindingField.js",
			"$POPATH/app/ux/form/field/MarkdownField.js",
			"$POPATH/app/ux/stackorganizer/column/IconRenderer.js",
			"$POPATH/app/ux/stackorganizer/column/GridIcon.js",
			"$POPATH/app/ux/stackorganizer/column/Icon.js",
			"$POPATH/app/ux/stackorganizer/column/MoveDown.js",
			"$POPATH/app/ux/stackorganizer/column/MoveUp.js",
			"$POPATH/app/ux/stackorganizer/column/Properties.js",
			"$POPATH/app/ux/stackorganizer/column/Specification.js",
			"$POPATH/app/ux/stackorganizer/column/TreeIcon.js",
			"$POPATH/app/ux/stackorganizer/store/Grid.js",
			"$POPATH/app/ux/stackorganizer/store/Tree.js",
			"$POPATH/app/ux/stackorganizer/view/ContentGrid.js",
			"$POPATH/app/ux/stackorganizer/view/StackTree.js",
			"$POPATH/app/ux/stackorganizer/view/ContentAccordion.js",
			"$POPATH/app/ux/stackorganizer/Panel.js",
			"$POPATH/app/ux/stackorganizer/Field.js",
			"$POPATH/app/ux/pdf/Viewer.js",
			"$POPATH/app/ux/gallery/view/DropBox.js",
			"$POPATH/app/ux/gallery/view/LinkPageGrid.js",
			"$POPATH/app/ux/gallery/view/LinkStackGrid.js",
			"$POPATH/app/ux/gallery/view/MediaGrid.js",
			"$POPATH/app/ux/data/model/WebBios.js",
			"$POPATH/app/ux/data/model/EditCode.js",
			"$POPATH/app/ux/data/store/EditCode.js",
			"$POPATH/app/ux/data/model/SnapOnIcon.js",
			"$POPATH/app/ux/data/store/SnapOnIcon.js",
			"$POPATH/app/ux/gallery/view/MediaEditor.js",
			"$POPATH/app/ux/gallery/Media.js",
			"$POPATH/app/ux/gallery/Field.js",
			"$POPATH/app/ux/folderorganizer/DmdEditor.js",
			"$POPATH/app/ux/folderorganizer/Tree.js",
			"$POPATH/app/ux/folderorganizer/Organizer.js",
			"$POPATH/app/ux/folderorganizer/FamilyOrganizer.js",
			"$POPATH/app/ux/widget/folderorganizer/Catalog.js",
			"$POPATH/app/ux/widget/folderorganizer/Contact.js",
			"$POPATH/app/ux/widget/folderorganizer/Manufacturer.js",
			"$POPATH/app/ux/widget/folderorganizer/Navigation.js",
			"$POPATH/app/ux/widget/folderorganizer/Pages.js",
			"$POPATH/app/ux/widget/folderorganizer/PageNavigation.js",
			"$POPATH/app/ux/widget/folderorganizer/Partsplexer.js",
			"$POPATH/app/ux/widget/folderorganizer/PortfolioMenu.js",
			"$POPATH/app/ux/widget/folderorganizer/Questionaries.js",
			"$POPATH/app/ux/call2action/LinkTabpanel.js",
			"$POPATH/app/ux/call2action/Tabpanel.js",
			"$POPATH/app/ux/call2action/Window.js",
			"$POPATH/app/ux/call2action/Grid.js",
			"$POPATH/app/ux/call2action/Field.js",
			"$POPATH/app/ux/call2action/Call2Action.js",
			"$POPATH/app/ux/form/field/XpdItemGrid.js",
			"$POPATH/app/ux/navigation/model/RolePermissionTree.js",
			"$POPATH/app/ux/navigation/controller/UserRole.js",
			"$POPATH/app/ux/navigation/view/UserRolePanel.js",
			"$POPATH/app/builder/diff/view/Main.js",
			"$POPATH/app/builder/save/view/Tree.js",
			"$POPATH/app/builder/save/model/Main.js",
			"$POPATH/app/builder/save/view/Main.js",
			"$POPATH/app/ux/file/util/Handler.js",
			"$POPATH/app/ux/file/view/Conflict.js",
			"$POPATH/app/ux/file/view/Explorer.js",
			"$POPATH/app/ux/file/model/Editor.js",
			"$POPATH/app/ux/file/view/Editor.js",
			"$POPATH/app/ux/serppreview/Container.js",
			"$POPATH/app/ux/serppreview/Field.js",
			"$POPATH/app/ux/plugin/Charcount.js",
			"$POPATH/app/ux/grid/DropZone.js",
			"$POPATH/app/ux/form/field/HtmlField.js",
			"$POPATH/app/ux/form/field/Combi.js",
			"$POPATH/app/ux/help/view/Manager.js",
			"$POPATH/app/ux/form/field/PrePostTextField.js",
			"$POPATH/app/ux/form/field/ComboInline.js",
			"$POPATH/app/ux/form/field/ComboSql.js",
			"$POPATH/app/ux/form/field/ComboTable.js",
			"$POPATH/app/ux/form/field/CollectionPicker.js",
			"$POPATH/app/ux/form/field/EnumComboBox.js",
			"$POPATH/app/ux/form/field/FileEditor.js",
			"$POPATH/app/ux/form/field/Registration.js",
			"$POPATH/app/ux/icon/model/DataView.js",
			"$POPATH/app/ux/icon/view/DataView.js",
			"$POPATH/app/ux/icon/view/Panel.js",
			"$POPATH/app/ux/form/field/IconPicker.js",
			"$POPATH/app/ux/hsvwheel/_js/wheel.js",
			"$POPATH/app/ux/hsvwheel/Picker.js",
			"$POPATH/app/ux/hsvwheel/Field.js",
			"$POPATH/app/ux/folderExplorer/Grid.js",
			"$POPATH/app/ux/folderExplorer/LogGrid.js",
			"$POPATH/app/ux/folderExplorer/LogWindow.js",
			"$POPATH/app/ux/svg/DrawRoom.js",
			"$POPATH/app/ux/link/Window.js",
			"$POPATH/app/ux/buffer/Proxy.js",
			"$POPATH/app/ux/buffer/Store.js",
			"$POPATH/app/ux/buffer/Grid.js",
			"$POPATH/app/ux/buffer/controller/Form.js",
			"$POPATH/app/ux/buffer/view/Form.js",
			"$POPATH/app/ux/form/field/EntityComboBox.js",
			"$POPATH/app/ux/form/field/EntityTagField.js",
			"$POPATH/app/ux/form/field/RestEntityPicker.js",
			"$POPATH/app/ux/form/field/LanguagePicker.js",
			"$POPATH/app/ux/form/field/ComboObjectType.js",
			"$POPATH/app/ux/data/dmd/Reader.js",
			"$POPATH/app/ux/data/dmd/Writer.js",
			"$POPATH/app/ux/data/dmd/Proxy.js",
			"$POPATH/app/ux/data/dmd/Store.js",
			"$POPATH/app/ux/grid/filter/Base.js",
			"$POPATH/app/ux/grid/filter/Array.js",
			"$POPATH/app/ux/grid/filter/String.js",
			"$POPATH/app/ux/grid/filter/Number.js",
			"$POPATH/app/ux/grid/filter/Enum.js",
			"$POPATH/app/ux/grid/filter/Entity.js",
			"$POPATH/app/ux/grid/filter/Boolean.js",
			"$POPATH/app/ux/grid/filter/Date.js",
			"$POPATH/app/ux/grid/filter/util.js",
			"$POPATH/app/ux/grid/filter/Toolbar.js",
			"$POPATH/app/ux/grid/filter/view/Edit.js",
			"$POPATH/app/ux/grid/filter/Menu.js",
			"$POPATH/app/ux/ToolbarDroppable.js",
			"$POPATH/app/component/form/field/db2/Text.js",
			"$POPATH/app/component/form/field/db2/Number.js",
			"$POPATH/app/component/form/field/db2/CheckBox.js",
			"$POPATH/app/component/form/field/db2/IsoTimestamp.js",
			"$POPATH/app/component/form/field/ComboBox.js",
			"$POPATH/app/component/form/field/Picker.js",
			"$POPATH/app/component/quick/view/Grid.js",
			"$POPATH/app/component/quick/controller/form.js",
			"$POPATH/app/component/quick/view/Form.js",
			"$POPATH/app/entity/util/DropZone.js",
			"$POPATH/app/entity/util/Handler.js",
			"$POPATH/app/pd/util/Handler.js",
			"$POPATH/app/pd/util/Cache.js",
			"$POPATH/app/ux/xpd/Handler.js",
			"$POPATH/app/entity/util/Layout.js",
			"$POPATH/app/notification/view/Item.js",
			"$POPATH/app/notification/view/List.js",
			"$POPATH/app/notification/view/Window.js",
			"$POPATH/app/notification/view/io/item.js",
			"$POPATH/app/entity/util/Toolbar.js",
			"$POPATH/app/entity/controller/Form.js",
			"$POPATH/app/entity/view/Form.js",
			"$POPATH/app/entity/view/Grid.js",
			"$POPATH/app/entity/view/Preview.js",
			"$POPATH/app/entity/store/Reader.js",
			"$POPATH/app/entity/store/Tree.js",
			"$POPATH/app/entity/view/Tree.js",
			"$POPATH/app/entity/view/Panel.js",
			"$POPATH/app/entity/plugin/Summary.js",
			"$POPATH/app/entity/plugin/Import.js",
			"$POPATH/app/entity/plugin/Chart.js",
			"$POPATH/app/io/view/Main.js",
			"$POPATH/app/ux/xpd/Panel.js",
			"$POPATH/app/ux/xpd/Field.js",
			"$POPATH/app/ux/folderorganizer/XpdEditor.js",
			"$POPATH/app/ux/form/field/TreePicker.js",
			"$POPATH/app/ux/form/field/Month.js",
			"$POPATH/app/ux/form/field/Link.js",
			"$POPATH/app/ux/form/field/Iframe.js",
			"$POPATH/app/login/controller/Template.js",
			"$POPATH/app/login/view/Template.js",
			"$POPATH/app/login/controller/Forgot.js",
			"$POPATH/app/login/view/Forgot.js",
			"$POPATH/app/login/controller/Expire.js",
			"$POPATH/app/login/view/Expire.js",
			"$POPATH/app/designer/ui/save/view/Tree.js",
			"$POPATH/app/designer/ui/save/view/Main.js",
			"$POPATH/app/menu/form/Link.js",
			"$POPATH/app/menu/form/App.js",
			"$POPATH/app/menu/save/view/Main.js",
			"$POPATH/app/top/controller/Settings.js",
			"$POPATH/app/top/view/Settings.js",
			"$POPATH/app/ux/GMapPanel.js",
			
		];

		if extra.is_empty() {

			extra = vec![
				"$POPATH/app/util/language-encoding.min.js"
			]
		}

		let mut op = Operation::new("Portfolio - system", "$POPATH/system.js", "$POPATH/system-debug.js");
		op.files = files;
		op.extra = extra;
		op.append = true;
		op.run()?;

		files = vec![
			"$POPATH/flags.css",
			"$POPATH/flatui.css",
			"$POPATH/ext-6.css",
			"$POPATH/app/ux/hsvwheel/_css/wheel.css",
			"$POPATH/app/pd/css/styles.css",
			"$POPATH/app/login/css/template.css"
		];

		extra = vec![
			"$POPATH/../ext-6.6.0/build/packages/charts/classic/neptune/resources/charts-all.css",
			"$POPATH/app/ux/simplemde/dist/simplemde.min.css"
		];

		let mut css_op = CssOperation::new("Portfolio - css", "$POPATH/system-styles.css", "$POPATH/system-styles-debug.css");
		css_op.files = files;
		css_op.extra = extra;
		css_op.append = true;
		css_op.run()?;

	}

	Ok(())
}

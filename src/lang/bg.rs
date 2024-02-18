lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Вашият работен плот може да бъде достъпен с този идентификационен код и парола."),
        ("connecting_status", "Свързване с RustDesk мрежата..."),
        ("not_ready_status", "Не е в готовност. Моля проверете мрежова връзка"),
        ("ID/Relay Server", "ID/Релейн сървър"),
        ("id_change_tip", "Само a-z, A-Z, 0-9 и _ (долна черта) символи са позволени. Първата буква трябва да е a-z, A-Z. С дължина мержу 6 и 16."),
        ("Slogan_tip", "Направено от сърце в този хаотичен свят!"),
        ("Build Date", "Дата на изграждане"),
        ("Audio Input", "Аудио вход"),
        ("Hardware Codec", "Хардуерен кодек"),
        ("ID Server", "ID сървър"),
        ("Relay Server", "Релейн сървър"),
        ("API Server", "API сървър"),
        ("invalid_http", "трябва да започва с http:// или https://"),
        ("server_not_support", "Все още не се поддържа от сървъра"),
        ("Password Required", "Изисква се парола"),
        ("Wrong Password", "Грешна парола"),
        ("Connection Error", "Грешка при свързване"),
        ("Login Error", "Грешка при вписване"),
        ("Show Hidden Files", "Показване на скрити файлове"),
        ("Refresh File", "Опресняване на файла"),
        ("Remote Computer", "Отдалечен компютър"),
        ("Local Computer", "Локален компютър"),
        ("Confirm Delete", "Потвърдете изтриването"),
        ("Multi Select", "Множествен избор"),
        ("Select All", "Избери всички"),
        ("Unselect All", "Деселектирай всички"),
        ("Empty Directory", "Празна директория"),
        ("Custom Image Quality", "Персонализирано качество на изображението"),
        ("Adjust Window", "Регулирай прозореца"),
        ("Insert Lock", "Поставете ключ"),
        ("Set Password", "Задайте парола"),
        ("OS Password", "Парола на Операционната система"),
        ("install_tip", "Поради UAC, RustDesk в някои случай не може да работи правилно като отдалечена достъп. За да заобиколите UAC, моля, щракнете върху бутона по-долу, за да инсталирате RustDesk в системата."),
        ("config_acc", "За да управлявате вашия работен плот дистанционно, трябва да предоставите на RustDesk разрешения \"Достъпност\"."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant RustDesk \"Screen Recording\" permissions."),
        ("Installation Path", "Инсталационен път"),
        ("agreement_tip", "Стартирайки инсталацията, вие приемате лицензионното споразумение."),
        ("Accept and Install", "Приемете и инсталирайте"),
        ("not_close_tcp_tip", "Не затваряйте този прозорец, докато използвате тунела"),
        ("Remote Host", "Отдалечен хост"),
        ("Remote Port", "Отдалечен порт"),
        ("Local Port", "Локален порт"),
        ("Local Address", "Локален адрес"),
        ("Change Local Port", "Промяна на локалният порт"),
        ("setup_server_tip", "За по-бърза връзка, моля направете свой собствен сървър"),
        ("Enter Remote ID", "Въведете дистанционно ID"),
        ("Auto Login", "Автоматично вписване (Валидно само ако зададете \"Заключване след края на сесията\")"),
        ("Change Path", "Промяна на пътя"),
        ("Create Folder", "Създай папка"),
        ("whitelist_tip", "Само IP от белия списък има достъп до мен"),
        ("verification_tip", "На регистрирания имейл адрес е изпратен код за потвърждение, въведете кода за потвърждение, за да продължите да влизате."),
        ("whitelist_sep", "Разделени със запетая, точка и запетая, интервали или нов ред"),
        ("Add Tag", "Добавяне на етикет"),
        ("Wrong credentials", "Wrong username or password"),
        ("Edit Tag", "Edit tag"),
        ("Forget Password", "Забравена парола"),
        ("Add to Favorites", "Добави към любими"),
        ("Remove from Favorites", "Премахване от любими"),
        ("Socks5 Proxy", "Socks5 прокси"),
        ("install_daemon_tip", "За стартиране с компютъра трябва да инсталирате системна услуга."),
        ("Are you sure to close the connection?", "Сигурни ли сте, че искате да затворите връзката?"),
        ("One-Finger Tap", "Докосване с един пръст"),
        ("Left Mouse", "Ляв бутон на мишката"),
        ("One-Long Tap", "Едно дълго докосване"),
        ("Two-Finger Tap", "Докосване с два пръста"),
        ("Right Mouse", "Десен бутон на мишката"),
        ("One-Finger Move", "Преместване с един пръст"),
        ("Double Tap & Move", "Докоснете два пъти и преместете"),
        ("Mouse Drag", "Плъзгане с мишката"),
        ("Three-Finger vertically", "Три пръста вертикално"),
        ("Mouse Wheel", "Колело на мишката"),
        ("Two-Finger Move", "Движение с два пръста"),
        ("Canvas Move", "Преместване на платното"),
        ("Pinch to Zoom", "Щипнете, за да увеличите"),
        ("Canvas Zoom", "Увеличение на платното"),
        ("Share Screen", "Сподели екран"),
        ("Screen Capture", "Заснемане на екрана"),
        ("Input Control", "Контрол на въвеждане"),
        ("Audio Capture", "Аудио записване"),
        ("File Connection", "Файлова връзка"),
        ("Screen Connection", "Свързване на екрана"),
        ("Open System Setting", "Отворете системната настройка"),
        ("android_input_permission_tip1", "За да може отдалечено устройство да управлява вашето Android устройство чрез мишка или докосване, трябва да разрешите на RustDesk да използва услугата \"Достъпност\"."),
        ("android_input_permission_tip2", "Моля, отидете на следващата страница с системни настройки, намерете и въведете [Installed Services], включете услугата [RustDesk Input]."),
        ("android_new_connection_tip", "Получена е нова заявка за контрол, която иска да контролира вашето текущо устройство."),
        ("android_service_will_start_tip", "Включването на \"Заснемане на екрана\" автоматично ще стартира услугата, позволявайки на други устройства да поискат връзка с вашето устройство."),
        ("android_stop_service_tip", "Затварянето на услугата автоматично ще затвори всички установени връзки."),
        ("android_version_audio_tip", "Текущата версия на Android не поддържа аудио заснемане, моля, актуализирайте устройството с Android 10 или по-нова версия."),
        ("android_start_service_tip", "Докоснете [Start service] или активирайте разрешение [Screen Capture], за да стартирате услугата за споделяне на екрана."),
        ("android_permission_may_not_change_tip", "Разрешенията за установени връзки може да не се променят незабавно, докато не се свържете отново."),
        ("doc_mac_permission", "https://Digi-Desk2.com/docs/en/manual/mac/#enable-permissions"),
        ("Ignore Battery Optimizations", "Игнорирай оптимизациите на батерията"),
        ("android_open_battery_optimizations_tip", "Ако искате да деактивирате тази функция, моля, отидете на следващата страница с настройки на приложението RustDesk, намерете и въведете [Battery], премахнете отметката от [Unrestricted]"),
        ("remote_restarting_tip", "Отдалеченото устройство се рестартира, моля, затворете това съобщение и се свържете отново с постоянна парола след известно време"),
        ("Exit Fullscreen", "Изход от цял екран"),
        ("Mobile Actions", "Мобилни действия"),
        ("Select Monitor", "Изберете монитор"),
        ("Control Actions", "Контролни действия"),
        ("Display Settings", "Настройки на дисплея"),
        ("Image Quality", "Качество на изображението"),
        ("Scroll Style", "Стил на превъртане"),
        ("Show Toolbar", "Показване на лентата с инструменти"),
        ("Hide Toolbar", "Скриване на лентата с инструменти"),
        ("Direct Connection", "Директна връзка"),
        ("Relay Connection", "Релейна връзка"),
        ("Secure Connection", "Защитена връзка"),
        ("Insecure Connection", "Незащитена връзка"),
        ("Dark Theme", "Тъмна тема"),
        ("Light Theme", "Светла тема"),
        ("Follow System", "Следвай системата"),
        ("Unlock Security Settings", "Отключи настройките за сигурност"),
        ("Unlock Network Settings", "Отключи мрежовите настройки"),
        ("Direct IP Access", "Директен IP достъп"),
        ("Audio Input Device", "Аудио входно устройство"),
        ("Use IP Whitelisting", "Използвайте бял списък с IP адреси"),
        ("Pin Toolbar", "Фиксиране на лентата с инструменти"),
        ("Unpin Toolbar", "Откачване на лентата с инструменти"),
        ("elevated_foreground_window_tip", "Текущият прозорец на отдалечения работен плот изисква по-високи привилегии за работа, така че временно не може да използва мишката и клавиатурата. Можете да поискате от отдалечения потребител да минимизира текущия прозорец или да щракнете върху бутона за повдигане в прозореца за управление на връзката. За да избегнете този проблем, се препоръчва да инсталирате софтуера на отдалеченото устройство."),
        ("Keyboard Settings", "Настройки на клавиатурата"),
        ("Full Access", "Пълен достъп"),
        ("Screen Share", "Споделяне на екрана"),
        ("JumpLink", "Преглед"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Моля, изберете екрана, който да бъде споделен (Работете от страна на партньора)."),
        ("One-time Password", "Еднократна парола"),
        ("hide_cm_tip", "Разрешете скриването само ако приемате сесии чрез парола и използвате постоянна парола"),
        ("wayland_experiment_tip", "Wayland support is in experimental stage, please use X11 if you require unattended access."),
        ("software_render_tip", "Ако използвате графична карта Nvidia под Linux и отдалеченият прозорец се затваря веднага след свързване, превключването към драйвера Nouveau с отворен код и изборът да използвате софтуерно изобразяване може да помогне. Изисква се рестартиране на софтуера."),
        ("config_input", "За да контролирате отдалечен работен плот с клавиатура, трябва да предоставите на RustDesk разрешения \"Input Monitoring\"."),
        ("config_microphone", "За да говорите дистанционно, трябва да предоставите на RustDesk разрешения \"Запис на звук\"."),
        ("request_elevation_tip", "Можете също така да поискате повишаване на привилегии, ако има някой от отдалечената страна."),
        ("Elevation Error", "Грешка при повишаване на привилегии"),
        ("still_click_uac_tip", "Все още изисква отдалеченият потребител да щракне върху OK в прозореца на UAC при стартиранят RustDesk."),
        ("Request Elevation", "Поискайте повишаване на привилегии"),
        ("wait_accept_uac_tip", "Моля, изчакайте отдалеченият потребител да приеме диалоговия прозорец на UAC."),
        ("Switch Sides", "Сменете страните"),
        ("Default View Style", "Стил на изглед по подразбиране"),
        ("Default Scroll Style", "Стил на превъртане по подразбиране"),
        ("Default Image Quality", "Качество на изображението по подразбиране"),
        ("Default Codec", "Кодек по подразбиране"),
        ("Other Default Options", "Други опции по подразбиране"),
        ("relay_hint_tip", "Може да не е възможно да се свържете директно; можете да опитате да се свържете чрез реле. Освен това, ако искате да използвате реле при първия си опит, добавете наставка \"/r\" към идентификатора или да изберете опцията \"Винаги свързване чрез реле\" в картата на последните сесии, ако съществува."),
        ("install_cert_tip", "Инсталирайте сертификат на RustDesk"),
        ("confirm_install_cert_tip", "Това е сертификат за тестване на RustDesk, на който може да се вярва. Сертификатът ще се използва за доверие и инсталиране на драйвери на RustDesk, когато е необходимо."),
        ("RDP Settings", "RDP настройки"),
        ("New Connection", "Ново свързване"),
        ("Your Device", "Вашето устройство"),
        ("empty_recent_tip", "Ами сега, няма скорошни сесии!\nВреме е да планирате нова."),
        ("empty_favorite_tip", "Все още нямате любими връстници?\nНека намерим някой, с когото да се свържете, и да го добавим към вашите любими!"),
        ("empty_lan_tip", "О, не, изглежда, че все още не сме открили връстници."),
        ("empty_address_book_tip", "Изглежда, че в момента няма изброени връстници във вашата адресна книга."),
        ("Empty Username", "Празно потребителско име"),
        ("Empty Password", "Празна парола"),
        ("identical_file_tip", "Този файл е идентичен с този на партньора."),
        ("show_monitors_tip", "Показване на мониторите в лентата с инструменти"),
        ("View Mode", "Режим на преглед"),
        ("login_linux_tip", "Трябва да влезете в отдалечен Linux акаунт, за да активирате X сесия на работния плот"),
        ("verify_rustdesk_password_tip", "Проверете RustDesk паролата"),
        ("remember_account_tip", "Запомнете този акаунт"),
        ("os_account_desk_tip", "Този акаунт се използва за влизане в отдалечената операционна система и позволява на десктоп сесията без глава"),
        ("OS Account", "Операционната система акаунт"),
        ("another_user_login_title_tip", "Друг потребител вече е влязъл"),
        ("another_user_login_text_tip", "Прекъснете връзката"),
        ("xorg_not_found_title_tip", "Xorg не е намерен"),
        ("xorg_not_found_text_tip", "Моля, инсталирайте Xorg"),
        ("no_desktop_title_tip", "Няма наличен работен плот"),
        ("no_desktop_text_tip", "Моля, инсталирайте работен плот GNOME"),
        ("System Sound", "Системен звук"),
        ("Copy Fingerprint", "Копиране на пръстов отпечатък"),
        ("no fingerprints", "Няма пръстови отпечатъци"),
        ("resolution_original_tip", "Оригинална резолюция"),
        ("resolution_fit_local_tip", "Напасване към локална разделителна способност"),
        ("resolution_custom_tip", "Персонализирана разделителна способност"),
        ("Accept and Elevate", "Приемете и повишаване на привилегии"),
        ("accept_and_elevate_btn_tooltip", "Приемете връзката и повишете UAC разрешенията."),
        ("clipboard_wait_response_timeout_tip", "Времето за изчакване на отговор за копиране изтече."),
        ("logout_tip", "Сигурни ли сте, че искате да излезете?"),
        ("exceed_max_devices", "Достигнахте максималния брой управлявани устройства."),
        ("Change Password", "Промяна на паролата"),
        ("Refresh Password", "Обнови паролата"),
        ("Grid View", "Мрежов изглед"),
        ("List View", "Списъчен изглед"),
        ("Toggle Tags", "Превключване на етикети"),
        ("pull_ab_failed_tip", "Неуспешно опресняване на адресната книга"),
        ("push_ab_failed_tip", "Неуспешно синхронизиране на адресната книга със сървъра"),
        ("synced_peer_readded_tip", "Устройствата, които са присъствали в последните сесии, ще бъдат синхронизирани обратно към адресната книга."),
        ("Change Color", "Промяна на цвета"),
        ("Primary Color", "Основен цвят"),
        ("HSV Color", "HSV цвят"),
        ("Installation Successful!", "Успешна инсталация!"),
        ("scam_title", "Възможно е да сте ИЗМАМЕНИ!"),
        ("scam_text1", "Ако разговаряте по телефона с някой, когото НЕ ПОЗНАВАТЕ и НЯМАТЕ ДОВЕРИЕ, който ви е помолил да използвате RustDesk и да стартирате услугата, не продължавайте и затворете незабавно."),
        ("scam_text2", "Те вероятно са измамник, който се опитва да открадне вашите пари или друга лична информация."),
        ("auto_disconnect_option_tip", "Автоматично затваряне на входящите сесии при неактивност на потребителя"),
        ("Connection failed due to inactivity", "Автоматично прекъсване на връзката поради неактивност"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Моля обновете RustDesk Server Pro на версия {} или по-нова!"),
        ("pull_group_failed_tip", "Неуспешно опресняване на групата"),
        ("doc_fix_wayland", "https://Digi-Desk2.com/docs/en/manual/linux/#x11-required"),
        ("display_is_plugged_out_msg", "Дисплеят е изключен, превключете на първия монитор."),
        ("elevated_switch_display_msg", "Превключете към основния монитор, защото множество монитори не се поддържат в потребителски режим с повишени права."),
        ("selinux_tip", "SELinux е активиран на вашето устройство, което може да попречи на RustDesk да работи правилно като контролирана страна."),
        ("id_input_tip", "Можете да въведете ID, директен IP адрес или домейн с порт (<domain>:<port>).\nАко искате да получите достъп до устройство на друг сървър, моля, добавете адреса на сървъра (<id>@<server_address >?key=<key_value>), например\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nАко искате да получите достъп до устройство на обществен сървър, моля, въведете \"<id>@public\" , ключът не е необходим за публичен сървър"),
        ("privacy_mode_impl_mag_tip", "Режим 1"),
        ("privacy_mode_impl_virtual_display_tip", "Режим 2"),
        ("idd_not_support_under_win10_2004_tip", "Индиректен драйвер за дисплей не се поддържа. Изисква се Windows 10, версия 2004 или по-нова."),
        ("switch_display_elevated_connections_tip", "Превключването към неосновен дисплей не се поддържа в режим на потребител с повишени права, когато има множество връзки. Моля, опитайте отново след инсталация, ако искате да контролирате няколко дисплея."),
        ("input_source_1_tip", "Входен източник 1"),
        ("input_source_2_tip", "Входен източник 2"),
        ("capture_display_elevated_connections_tip", "Заснемането на множество дисплеи не се поддържа в потребителския режим с повишени права. Моля, опитайте отново след инсталация, ако искате да контролирате няколко дисплея."),
        ("swap-left-right-mouse", "Разменете левия и десния бутон на мишката"),
        ("2FA code", "Код за Двуфакторна удостоверяване"),
        ("enable-2fa-title", "Активиране на двуфакторно удостоверяване"),
        ("enable-2fa-desc", "Моля, настройте вашия удостоверител сега. Можете да използвате приложение за удостоверяване като Authy, Microsoft или Google Authenticator на вашия телефон или настолен компютър.\n\nСканирайте QR кода с вашето приложение и въведете кода, който приложението ви показва, за да активирате двуфакторно удостоверяване."),
        ("wrong-2fa-code", "е може да се потвърди кодът. Проверете дали настройките за код и локалното време са правилни"),
        ("enter-2fa-title", "Двуфакторно удостоверяване"),
    ].iter().cloned().collect();
}

settings-library-watch-section-desc = Обнаруживать файлы, добавленные вне skwd-wall, и поддерживать библиотеку в актуальном состоянии.
settings-library-watch-fallback-label = Резервный опрос
settings-library-watch-fallback-desc = Проверять только те папки библиотеки, за которыми не может следить встроенное отслеживание файлов. Включите для сетевых или FUSE-монтирований, которые пропускают изменения, затем перезапустите skwd-walld.
settings-library-watch-interval-label = Интервал опроса
settings-library-watch-interval-desc = Столько секунд ждать между ограниченными проверками. Меньшие значения быстрее находят изменения, но чаще читают файловую систему. После изменения перезапустите skwd-walld.
settings-library-watch-unknown-label = Состояние отслеживания недоступно
settings-library-watch-unknown-desc = Эта служба не сообщает о состоянии отслеживания библиотеки. Обновите или перезапустите skwd-walld.
settings-library-watch-poll-failed-label = Опрос не может прочитать папку библиотеки
settings-library-watch-poll-failed-desc = Проверьте, что все настроенные папки библиотеки смонтированы и доступны для чтения. Повторный опрос через { $interval } с.
settings-library-watch-polling-label = Резервный опрос активен
settings-library-watch-polling-desc = Встроенное отслеживание не работает для { $count ->
    [one] { $count } папки библиотеки
    [few] { $count } папок библиотеки
    [many] { $count } папок библиотеки
   *[other] { $count } папки библиотеки
    }. Каждые { $interval } с проверяется до { $budget } записей. Последняя успешная синхронизация: { $convergence }.
settings-library-watch-recovering-label = Встроенное отслеживание восстановлено
settings-library-watch-recovering-desc = Встроенное отслеживание снова активно. Полное переходное сканирование ещё идёт, прежде чем библиотека будет считаться актуальной.
settings-library-watch-unavailable-label = Отслеживание библиотеки недоступно
settings-library-watch-unavailable-desc = Встроенное отслеживание файлов не работает, а резервный опрос выключен. Включите резервный опрос, затем перезапустите skwd-walld.
settings-library-watch-recovered-label = Встроенное отслеживание восстановлено
settings-library-watch-recovered-desc = Встроенное отслеживание и переходное сканирование актуальны. Последняя успешная синхронизация: { $convergence }.
settings-library-watch-native-label = Встроенное отслеживание файлов
settings-library-watch-native-desc = События файловой системы активны для всех папок библиотеки. Опрос простаивает.
settings-library-watch-convergence-never = Ещё не завершено
settings-library-watch-convergence-seconds = { $value ->
    [one] { $value } секунду назад
    [few] { $value } секунды назад
    [many] { $value } секунд назад
   *[other] { $value } секунды назад
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] { $value } минуту назад
    [few] { $value } минуты назад
    [many] { $value } минут назад
   *[other] { $value } минуты назад
    }
settings-library-watch-convergence-hours = { $value ->
    [one] { $value } час назад
    [few] { $value } часа назад
    [many] { $value } часов назад
   *[other] { $value } часа назад
    }

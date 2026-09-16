settings-library-watch-section-desc = يكتشف الملفات المضافة من خارج skwd-wall ويبقي المكتبة محدّثة.
settings-library-watch-fallback-label = الفحص الدوري الاحتياطي
settings-library-watch-fallback-desc = يفحص فقط مجلدات المكتبة التي تعجز مراقبة الملفات الأصلية عن مراقبتها. فعّله لنقاط تركيب الشبكة أو FUSE التي تفوّت التغييرات، ثم أعد تشغيل skwd-walld.
settings-library-watch-interval-label = فترة الفحص الدوري
settings-library-watch-interval-desc = ينتظر هذا العدد من الثواني بين الفحوصات المحدودة. القيم الأقل تجد التغييرات أسرع لكنها تقرأ نظام الملفات أكثر. أعد تشغيل skwd-walld بعد تغييره.
settings-library-watch-unknown-label = حالة المراقبة غير متاحة
settings-library-watch-unknown-desc = لا تبلّغ هذه الخدمة عن حالة مراقبة المكتبة. حدّث skwd-walld أو أعد تشغيله.
settings-library-watch-poll-failed-label = يتعذر على الفحص الدوري قراءة مجلد مكتبة
settings-library-watch-poll-failed-desc = تأكد من أن كل مجلد مكتبة مضبوط مركّب وقابل للقراءة. سيعيد الفحص الدوري المحاولة بعد { $interval } ثانية.
settings-library-watch-polling-label = الفحص الدوري الاحتياطي نشط
settings-library-watch-polling-desc = تعذّرت المراقبة الأصلية في { $count ->
    [zero] صفر من مجلدات المكتبة
    [one] مجلد مكتبة واحد
    [two] مجلدي مكتبة
    [few] { $count } مجلدات مكتبة
    [many] { $count } مجلدًا من مجلدات المكتبة
   *[other] { $count } مجلدٍ من مجلدات المكتبة
    }. يُفحص ما يصل إلى { $budget } إدخال كل { $interval } ثانية. آخر مزامنة ناجحة: { $convergence }.
settings-library-watch-recovering-label = تعافت المراقبة الأصلية
settings-library-watch-recovering-desc = المراقب الأصلي نشط من جديد. لا يزال فحص التسليم الكامل جاريًا قبل اعتبار المكتبة محدّثة.
settings-library-watch-unavailable-label = مراقبة المكتبة غير متاحة
settings-library-watch-unavailable-desc = تعذّرت مراقبة الملفات الأصلية والفحص الدوري الاحتياطي متوقف. فعّل «الفحص الدوري الاحتياطي»، ثم أعد تشغيل skwd-walld.
settings-library-watch-recovered-label = استُعيدت المراقبة الأصلية
settings-library-watch-recovered-desc = المراقب الأصلي وفحص التسليم الخاص به محدّثان. آخر مزامنة ناجحة: { $convergence }.
settings-library-watch-native-label = مراقبة الملفات الأصلية
settings-library-watch-native-desc = أحداث نظام الملفات نشطة لكل مجلد مكتبة. الفحص الدوري خامل.
settings-library-watch-convergence-never = لم تكتمل بعد
settings-library-watch-convergence-seconds = { $value ->
    [zero] الآن
    [one] منذ ثانية واحدة
    [two] منذ ثانيتين
    [few] منذ { $value } ثوانٍ
    [many] منذ { $value } ثانيةً
   *[other] منذ { $value } ثانيةٍ
    }
settings-library-watch-convergence-minutes = { $value ->
    [zero] منذ أقل من دقيقة
    [one] منذ دقيقة واحدة
    [two] منذ دقيقتين
    [few] منذ { $value } دقائق
    [many] منذ { $value } دقيقةً
   *[other] منذ { $value } دقيقةٍ
    }
settings-library-watch-convergence-hours = { $value ->
    [zero] منذ أقل من ساعة
    [one] منذ ساعة واحدة
    [two] منذ ساعتين
    [few] منذ { $value } ساعات
    [many] منذ { $value } ساعةً
   *[other] منذ { $value } ساعةٍ
    }

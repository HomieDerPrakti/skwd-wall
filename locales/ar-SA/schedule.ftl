schedule-masthead = الجدول  /  { $selected }
schedule-masthead-fallback = القواعد
schedule-index-title = الجدول
schedule-index-subtitle = تُقرأ القواعد من الأعلى إلى الأسفل. أول قاعدة مطابقة هي التي تُطبَّق.
schedule-index-empty-title = لا توجد قواعد جدولة
schedule-index-empty-hint = أنشئ واحدة أدناه للبدء.
schedule-state-title = حالة الجدول
schedule-state-enabled = مفعّل
schedule-state-disabled = معطّل
schedule-state-enabled-desc = هذه المجموعة المرتبة من القواعد نشطة.
schedule-state-disabled-desc = تبقى القواعد محفوظة، لكن لن يعمل أي منها.
schedule-new-rule = قاعدة جديدة
schedule-create-rule = إنشاء قاعدة
schedule-unnamed-rule = قاعدة بلا اسم
schedule-random-wallpaper = خلفية عشوائية
schedule-rule-on = تشغيل
schedule-rule-off = إيقاف
schedule-rule-state = حالة القاعدة
schedule-empty-eyebrow = الجدول / القواعد
schedule-empty-title = أنشئ جدولًا
schedule-empty-desc = أنشئ قاعدة في الفهرس. تعمل القواعد من الأعلى إلى الأسفل وتُحفظ أثناء التحرير.
schedule-rule-eyebrow = الجدول / قاعدة
schedule-status-migrated = مُرحَّلة · محفوظة
schedule-status-saved = محفوظة
schedule-status-demo = تجريبية · غير محفوظة
schedule-summary-when-label = عندما
schedule-summary-then-label = عندئذٍ
schedule-apply-label = تطبيق { $target }
schedule-delete-rule = حذف القاعدة
schedule-identity-title = اسم القاعدة
schedule-identity-desc = أعطِ القاعدة اسمًا قصيرًا ليسهل تمييزها في قائمة الأولويات.
schedule-rule-name-placeholder = اسم القاعدة
schedule-wallpaper-title = الخلفية
schedule-wallpaper-desc = أدخل random أو مفتاح مكتبة أو playlist:ID. تُستخدم القيمة فقط عندما تكون هذه القاعدة أول قاعدة مطابقة.
schedule-wallpaper-placeholder = random, static:foo.png, video:clip.mp4, we:123, playlist:12
schedule-random-action = عشوائي
schedule-theme-title = السمة
schedule-theme-desc = أبقِ السمة الحالية، أو بدّلها عندما تكون هذه القاعدة أول قاعدة مطابقة.
schedule-theme-keep = إبقاء الحالية
schedule-theme-light = فاتحة
schedule-theme-dark = داكنة
schedule-conditions-title = متى ينبغي أن تعمل هذه القاعدة؟
schedule-conditions-desc = ابدأ بشروط عادية. أضف مجموعة متداخلة فقط عند الحاجة إلى منطق مختلط بين الكل/أي.
schedule-editor-add-title = إضافة إلى هذه المجموعة
schedule-editor-expression = التعبير
schedule-editor-group-title = إعدادات المجموعة
schedule-editor-fallback-title = تحرير الشروط
schedule-editor-desc = تُحفظ التغييرات فورًا.
schedule-editor-idle-desc = حدد عنصرًا في قائمة الشروط لإظهار عناصر التحكم به.
schedule-editor-idle-hint = حدد شرطًا أو مجموعة أعلاه لتحريره.
schedule-choose-expression = اختر تعبيرًا لتحريره.
schedule-setup-title = إعداد شروط هذه القاعدة
schedule-setup-desc = هذه القاعدة غير المكتملة لا تملك شجرة شروط صالحة. ابدأ من جديد، ثم أضف الشروط بلغة بسيطة.
schedule-setup-action = البدء بدون شروط
schedule-groups-hint = استخدم المجموعات فقط عندما تحتاج قاعدة واحدة إلى مزيج من منطق الكل/أي.
schedule-add-title = ماذا ينبغي أن تتحقق منه هذه المجموعة؟
schedule-add-desc = اختر شرطًا، أو أضف مجموعة متداخلة عندما تحتاج هذه القاعدة إلى منطق مختلط بين الكل/أي.
schedule-add-group-all = مجموعة متداخلة: مطابقة الكل
schedule-add-group-any = مجموعة متداخلة: مطابقة أي
schedule-add-condition-or-group = إضافة شرط أو مجموعة
schedule-group-match-all = مطابقة الكل
schedule-group-match-any = مطابقة أي
schedule-group-include = تضمين هذه المجموعة
schedule-group-exclude = استبعاد هذه المجموعة
schedule-group-add-inside = إضافة داخل هذه المجموعة
schedule-group-remove = إزالة المجموعة
schedule-group-all-detail =
    { $count ->
        [zero] لا توجد عناصر أدناه للمطابقة
        [one] يجب أن يطابق العنصر أدناه
        [two] يجب أن يطابق كلا العنصرين أدناه
        [few] يجب أن تطابق العناصر الـ{ $count } أدناه كلها
        [many] يجب أن يطابق كل عنصر من العناصر الـ{ $count } أدناه
       *[other] يجب أن تطابق جميع العناصر الـ{ $count } أدناه
    }
schedule-group-any-detail =
    { $count ->
        [zero] لا توجد عناصر أدناه للمطابقة
        [one] يجب أن يطابق العنصر أدناه
        [two] يجب أن يطابق أحد العنصرين أدناه على الأقل
        [few] يجب أن يطابق عنصر واحد على الأقل من العناصر الـ{ $count } أدناه
        [many] يجب أن يطابق واحد على الأقل من العناصر الـ{ $count } أدناه
       *[other] يجب أن يطابق عنصر واحد على الأقل من بين العناصر الـ{ $count } أدناه
    }
schedule-node-condition = شرط
schedule-node-root = شروط القاعدة
schedule-node-nested = مجموعة متداخلة
schedule-node-must-match = يجب أن يتحقق هذا الشرط
schedule-node-must-not-match = يجب ألا يتحقق هذا الشرط
schedule-node-always = تعمل في كل مرة
schedule-node-never = لا تعمل أبدًا
schedule-node-match-all = مطابقة كل هذه
schedule-node-match-any = مطابقة أي من هذه
schedule-node-not-all = عدم مطابقة كل هذه
schedule-node-match-none = عدم مطابقة أي من هذه
schedule-node-empty-always = لم تُضف شروط - تعمل هذه القاعدة حاليًا في كل مرة
schedule-node-empty-never = لم تُضف شروط - هذه القاعدة معطّلة حاليًا
schedule-node-editing = قيد التحرير
schedule-node-edit = تحرير
schedule-if = إذا
schedule-and = و
schedule-or = أو
schedule-must-match = يجب أن يتحقق
schedule-must-not-match = يجب ألا يتحقق
schedule-remove-condition = إزالة الشرط
schedule-kind-weekday = يوم الأسبوع
schedule-kind-time-range = نطاق زمني
schedule-kind-after-time = بعد وقت
schedule-kind-before-time = قبل وقت
schedule-kind-date = تاريخ أو نطاق تواريخ
schedule-kind-year = السنة
schedule-kind-weather = الطقس
schedule-kind-power = مصدر الطاقة
schedule-kind-battery = مستوى البطارية
schedule-kind-output = شاشة متصلة
schedule-kind-output-count = عدد الشاشات / الإرساء
schedule-kind-unsupported = شرط غير مدعوم
schedule-sentence-day = اليوم هو { $list }
schedule-sentence-time-between = الوقت بين { $from } و { $to }
schedule-sentence-time-cmp = الوقت { $relation } { $at }
schedule-sentence-date = التاريخ هو { $date }
schedule-sentence-year = السنة { $relation } { $year }
schedule-sentence-weather = الطقس { $list }
schedule-sentence-power = مصدر الطاقة هو { $source }
schedule-sentence-battery = البطارية { $relation } { $percent }%
schedule-sentence-output = الشاشة { $output } متصلة
schedule-sentence-output-count = عدد الشاشات المتصلة { $relation } { $count }
schedule-sentence-needs-setup = تحتاج هذه القاعدة إلى إعداد شروطها
schedule-sentence-unsupported = شرط غير مدعوم: { $raw }
schedule-relation-after = بعد
schedule-relation-before = قبل
schedule-relation-at-least = على الأقل
schedule-relation-at-most = على الأكثر
schedule-relation-exactly = بالضبط
schedule-relation-more-than = أكثر من
schedule-relation-fewer-than = أقل من
schedule-date-range-joiner = إلى
schedule-list-nothing = لا شيء
schedule-list-pair = { $first } أو { $second }
schedule-list-many = { $items }، أو { $last }
schedule-summary-needs-setup = يحتاج إلى إعداد الشروط
schedule-summary-negated = ليس: { $summary }
schedule-summary-always = دائمًا
schedule-summary-never = أبدًا
schedule-summary-match-all = مطابقة الكل
schedule-summary-match-any = مطابقة أي من
schedule-summary-group = { $operator } { $count ->
        [zero] بلا شروط
        [one] شرط واحد
        [two] شرطان
        [few] { $count } شروط
        [many] { $count } شرطًا
       *[other] { $count } شرط
    }
schedule-from = من
schedule-to = إلى
schedule-cmp-after = بعد
schedule-cmp-before = قبل
schedule-sunrise = الشروق
schedule-sunset = الغروب
schedule-solar-before = قبل { $solar } بـ { $minutes } دقيقة
schedule-solar-after = بعد { $solar } بـ { $minutes } دقيقة
schedule-time-placeholder = HH:MM أو sunset-30
schedule-time-hint = استخدم وقتًا بصيغة الساعة، أو sunrise، أو sunset، أو أضف إزاحة مثل sunset-30 أو sunrise+45.
schedule-weather-hint = يستخدم موقع الطقس المضبوط في الإعدادات.
schedule-date-placeholder = 12-25 أو 2026-12-25 أو 07-01..07-14
schedule-date-hint = استخدم MM-DD أو YYYY-MM-DD، ويمكن جعلها نطاقًا بوصل الطرفين بالفاصل .. . قد تمتد النطاقات المتكررة عبر بداية السنة الجديدة.
schedule-year-exact = بالضبط
schedule-year-at-least = على الأقل
schedule-year-at-most = على الأكثر
schedule-comparison-exact = بالضبط
schedule-comparison-at-least = على الأقل
schedule-comparison-at-most = على الأكثر
schedule-power-battery = طاقة البطارية
schedule-power-external = طاقة خارجية
schedule-battery-hint = النسبة المئوية من بطارية النظام. حالة البطارية المجهولة تُعد غير مطابقة.
schedule-output-placeholder = DP-1 أو eDP-1
schedule-output-hint = طابق اسم المخرج المتصل بالضبط. اعكس هذا الشرط للمطابقة عند غيابه.
schedule-output-count-hint = شاشتان على الأقل هي القاعدة المعتادة للحاسوب المحمول المرسى.
schedule-raw-placeholder = شرط خام
schedule-raw-hint = يُحفظ هذا الشرط كما كُتب بالضبط.
schedule-day-mon = الاثنين
schedule-day-tue = الثلاثاء
schedule-day-wed = الأربعاء
schedule-day-thu = الخميس
schedule-day-fri = الجمعة
schedule-day-sat = السبت
schedule-day-sun = الأحد
schedule-weather-clear = صافٍ
schedule-weather-sunny = مشمس
schedule-weather-cloudy = غائم
schedule-weather-rainy = ممطر
schedule-weather-snowy = مثلج
schedule-weather-stormy = عاصف
schedule-weather-foggy = ضبابي
schedule-weather-windy = كثير الرياح

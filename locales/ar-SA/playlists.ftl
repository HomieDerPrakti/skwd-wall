playlists-unnamed = (بلا اسم)
playlists-generated-name = قائمة تشغيل { $number }
playlists-new-name = اسم قائمة التشغيل الجديدة
playlists-name = اسم قائمة التشغيل
playlists-play-now = تشغيل الآن
playlists-delete = حذف
playlists-type-curated = يدوية
playlists-type-smart = مرشّحة
playlists-order-shuffle = عشوائي
playlists-order-sequential = متسلسل
playlists-every = كل
playlists-seconds-minimum = ثانية (الحد الأدنى 5)
playlists-all = الكل
playlists-favourites = المفضلات
playlists-filter-example = مثل type:video color:blue tag:cat,-anime
playlists-empty-smart = لا توجد خلفيات مطابقة لهذا المرشح بعد.
playlists-empty-curated = قائمة التشغيل هذه فارغة. افتح بطاقة خلفية واختر قائمة التشغيل لإضافتها.
playlists-stop-all = ■ إيقاف الكل
playlists-card-add-title = إضافة إلى قائمة تشغيل
playlists-card-none = لا توجد قوائم تشغيل بعد. سمِّ واحدة لإنشائها مع هذه الخلفية مضافة إليها:
playlists-card-new = أو أنشئ قائمة تشغيل أخرى مع هذه الخلفية:
playlists-masthead = قوائم التشغيل  /  { $selected }
playlists-masthead-fallback = المكتبة
playlists-index-title = قوائم التشغيل
playlists-index-subtitle = تحتوي القائمة اليدوية على خلفيات تختارها بنفسك. أما القائمة المرشّحة فتبقى متزامنة مع قواعدها.
playlists-index-live-detail = { $kind } · نشطة على { $outputs }
playlists-index-count-detail =
    { $kind }  ·  { $count ->
        [zero] لا توجد خلفيات
        [one] خلفية واحدة
        [two] خلفيتان
        [few] { $count } خلفيات
        [many] { $count } خلفيةً
       *[other] { $count } خلفية
    }
playlists-index-empty-title = لا توجد قوائم تشغيل بعد
playlists-index-empty-hint = أنشئ واحدة أدناه للبدء.
playlists-new-playlist = قائمة تشغيل جديدة
playlists-live-routing = الشاشات النشطة
playlists-active-assignments =
    { $count ->
        [zero] لا توجد تعيينات نشطة
        [one] تعيين نشط واحد
        [two] تعيينان نشطان
        [few] { $count } تعيينات نشطة
        [many] { $count } تعيينًا نشطًا
       *[other] { $count } تعيين نشط
    }
playlists-library = قوائم التشغيل المحفوظة
playlists-library-stats = { $count } محفوظة · لا شيء نشط
playlists-live-results = الخلفيات المطابقة
playlists-the-sequence = ترتيب التشغيل
playlists-sequence-smart-hint = الخلفيات المطابقة للمرشح الحالي، بالترتيب المعروض.
playlists-sequence-curated-hint = تُشغَّل الخلفيات من الأعلى إلى الأسفل. اسحبها لتغيير الترتيب.
playlists-empty-smart-hint = لا توجد خلفيات مطابقة لهذه القواعد. غيّر المرشح ليشمل المزيد.
playlists-empty-curated-hint = افتح بطاقة خلفية واختر قائمة التشغيل لإضافة أول خلفية.
playlists-live-badge = نشطة
playlists-empty-eyebrow = قوائم التشغيل / المكتبة
playlists-empty-title = أنشئ تناوبًا
playlists-empty-choose = اختر قائمة تشغيل من الفهرس
playlists-empty-desc = أو أنشئ قائمة تشغيل يدوية جديدة وابدأ بإضافة الخلفيات.
playlists-kind-smart = قائمة تشغيل مرشّحة
playlists-kind-curated = قائمة تشغيل يدوية
playlists-state-live = نشطة / { $outputs }
playlists-state-query-ready = المرشح جاهز
playlists-state-ready =
    { $count ->
        [zero] { $padded } لا خلفيات جاهزة
        [one] { $padded } خلفية واحدة جاهزة
        [two] { $padded } خلفيتان جاهزتان
        [few] { $padded } خلفيات جاهزة
        [many] { $padded } خلفيةً جاهزة
       *[other] { $padded } خلفية جاهزة
    }
playlists-edition-eyebrow = قوائم التشغيل / تحرير
playlists-kind-tagline = { $kind } · جاهزة للتعيين على الشاشات
playlists-stop-edition = إيقاف قائمة التشغيل
playlists-colour-notes = الألوان
playlists-live-collection-title = قائمة تشغيل مرشّحة
playlists-live-collection-desc = تتحدث هذه القائمة تلقائيًا وفق مرشحات المصدر والوسوم والدقة واللون المحددة.
playlists-definition-title = تفاصيل قائمة التشغيل
playlists-definition-desc = عيّن الاسم واختر ما إذا كانت الخلفيات تُضاف يدويًا أو تُختار بمرشح.
playlists-playback-title = التشغيل
playlists-playback-desc = اختر الترتيب ومدة بقاء كل خلفية على الشاشة.
playlists-routing-title = التطبيق على الشاشات
playlists-routing-desc = اختر أين تعمل قائمة التشغيل هذه. تحديد كل الشاشات ينشئ تعيينًا مشتركًا واحدًا.
playlists-routing-all = كل الشاشات المتصلة
playlists-routing-live = التعيين النشط
playlists-routing-none = لا يوجد تعيين نشط
playlists-demo-name = أجرأ مقطع

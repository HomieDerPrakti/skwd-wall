settings-library-watch-section-desc = skwd-wall سے باہر شامل کی گئی فائلیں پہچانیں اور لائبریری کو تازہ رکھیں۔
settings-library-watch-fallback-label = پولنگ فال بیک
settings-library-watch-fallback-desc = صرف وہ لائبریری فولڈر جانچیں جنہیں نیٹو فائل واچنگ نہیں دیکھ سکتی۔ نیٹ ورک یا FUSE ماؤنٹس کے لیے جو تبدیلیاں چھوڑ دیتے ہیں اسے فعال کریں، پھر skwd-walld دوبارہ شروع کریں۔
settings-library-watch-interval-label = پولنگ کا وقفہ
settings-library-watch-interval-desc = محدود جانچوں کے درمیان اتنے سیکنڈ انتظار کریں۔ کم قدریں تبدیلیاں جلد پکڑتی ہیں مگر فائل سسٹم زیادہ بار پڑھتی ہیں۔ بدلنے کے بعد skwd-walld دوبارہ شروع کریں۔
settings-library-watch-unknown-label = واچر کی حالت دستیاب نہیں
settings-library-watch-unknown-desc = یہ ڈیمن لائبریری واچ کی حالت رپورٹ نہیں کرتا۔ skwd-walld اپ ڈیٹ یا دوبارہ شروع کریں۔
settings-library-watch-poll-failed-label = پولنگ ایک لائبریری فولڈر نہیں پڑھ سکتی
settings-library-watch-poll-failed-desc = جانچیں کہ ہر ترتیب دیا گیا لائبریری فولڈر ماؤنٹ اور پڑھنے کے قابل ہے۔ پولنگ { $interval } سیکنڈ میں دوبارہ کوشش کرے گی۔
settings-library-watch-polling-label = پولنگ فال بیک فعال
settings-library-watch-polling-desc = نیٹو واچنگ { $count ->
    [one] { $count } لائبریری فولڈر
   *[other] { $count } لائبریری فولڈروں
    } کے لیے ناکام ہوئی۔ ہر { $interval } سیکنڈ میں زیادہ سے زیادہ { $budget } اندراج جانچے جاتے ہیں۔ آخری کامیاب ہم آہنگی: { $convergence }۔
settings-library-watch-recovering-label = نیٹو واچنگ بحال ہو گئی
settings-library-watch-recovering-desc = نیٹو واچر دوبارہ فعال ہے۔ لائبریری کو تازہ قرار دینے سے پہلے مکمل ہینڈ آف اسکین ابھی چل رہا ہے۔
settings-library-watch-unavailable-label = لائبریری واچنگ دستیاب نہیں
settings-library-watch-unavailable-desc = نیٹو فائل واچنگ ناکام ہوئی اور پولنگ فال بیک بند ہے۔ پولنگ فال بیک فعال کریں، پھر skwd-walld دوبارہ شروع کریں۔
settings-library-watch-recovered-label = نیٹو واچنگ بحال
settings-library-watch-recovered-desc = نیٹو واچر اور اس کا ہینڈ آف اسکین تازہ ہیں۔ آخری کامیاب ہم آہنگی: { $convergence }۔
settings-library-watch-native-label = نیٹو فائل واچنگ
settings-library-watch-native-desc = ہر لائبریری فولڈر کے لیے فائل سسٹم ایونٹس فعال ہیں۔ پولنگ غیر فعال ہے۔
settings-library-watch-convergence-never = ابھی مکمل نہیں ہوئی
settings-library-watch-convergence-seconds = { $value ->
    [one] { $value } سیکنڈ پہلے
   *[other] { $value } سیکنڈ پہلے
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] { $value } منٹ پہلے
   *[other] { $value } منٹ پہلے
    }
settings-library-watch-convergence-hours = { $value ->
    [one] { $value } گھنٹہ پہلے
   *[other] { $value } گھنٹے پہلے
    }

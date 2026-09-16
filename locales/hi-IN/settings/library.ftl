settings-library-watch-section-desc = skwd-wall के बाहर जोड़ी गई फ़ाइलें पहचानें और लाइब्रेरी अद्यतन रखें।
settings-library-watch-fallback-label = पोलिंग फ़ॉलबैक
settings-library-watch-fallback-desc = केवल उन लाइब्रेरी फ़ोल्डरों की जाँच करें जिन्हें नेटिव फ़ाइल निगरानी देख नहीं सकती। बदलाव छूटने वाले नेटवर्क या FUSE माउंट के लिए इसे चालू करें, फिर skwd-walld पुनः आरंभ करें।
settings-library-watch-interval-label = पोलिंग अंतराल
settings-library-watch-interval-desc = सीमित जाँचों के बीच इतने सेकंड प्रतीक्षा करें। कम मान बदलाव जल्दी पकड़ते हैं पर फ़ाइल सिस्टम को अधिक बार पढ़ते हैं। इसे बदलने के बाद skwd-walld पुनः आरंभ करें।
settings-library-watch-unknown-label = निगरानी की स्थिति अनुपलब्ध
settings-library-watch-unknown-desc = यह डेमन लाइब्रेरी निगरानी की स्थिति नहीं बताता। skwd-walld अपडेट या पुनः आरंभ करें।
settings-library-watch-poll-failed-label = पोलिंग कोई लाइब्रेरी फ़ोल्डर पढ़ नहीं पा रही
settings-library-watch-poll-failed-desc = जाँचें कि हर कॉन्फ़िगर किया गया लाइब्रेरी फ़ोल्डर माउंट और पठनीय है। पोलिंग { $interval } सेकंड में फिर से प्रयास करेगी।
settings-library-watch-polling-label = पोलिंग फ़ॉलबैक सक्रिय
settings-library-watch-polling-desc = { $count ->
    [one] { $count } लाइब्रेरी फ़ोल्डर
   *[other] { $count } लाइब्रेरी फ़ोल्डरों
    } में नेटिव निगरानी विफल रही। हर { $interval } सेकंड में अधिकतम { $budget } प्रविष्टियाँ जाँची जाती हैं। अंतिम सफल समन्वय: { $convergence }।
settings-library-watch-recovering-label = नेटिव निगरानी बहाल हुई
settings-library-watch-recovering-desc = नेटिव निगरानी फिर से सक्रिय है। लाइब्रेरी को अद्यतन घोषित करने से पहले पूरा हैंड-ऑफ़ स्कैन अभी चल रहा है।
settings-library-watch-unavailable-label = लाइब्रेरी निगरानी अनुपलब्ध
settings-library-watch-unavailable-desc = नेटिव फ़ाइल निगरानी विफल रही और पोलिंग फ़ॉलबैक बंद है। पोलिंग फ़ॉलबैक चालू करें, फिर skwd-walld पुनः आरंभ करें।
settings-library-watch-recovered-label = नेटिव निगरानी बहाल
settings-library-watch-recovered-desc = नेटिव निगरानी और उसका हैंड-ऑफ़ स्कैन अद्यतन हैं। अंतिम सफल समन्वय: { $convergence }।
settings-library-watch-native-label = नेटिव फ़ाइल निगरानी
settings-library-watch-native-desc = हर लाइब्रेरी फ़ोल्डर के लिए फ़ाइल सिस्टम इवेंट सक्रिय हैं। पोलिंग निष्क्रिय है।
settings-library-watch-convergence-never = अभी पूरा नहीं हुआ
settings-library-watch-convergence-seconds = { $value ->
    [one] { $value } सेकंड पहले
   *[other] { $value } सेकंड पहले
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] { $value } मिनट पहले
   *[other] { $value } मिनट पहले
    }
settings-library-watch-convergence-hours = { $value ->
    [one] { $value } घंटा पहले
   *[other] { $value } घंटे पहले
    }

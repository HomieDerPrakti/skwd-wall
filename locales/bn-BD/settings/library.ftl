settings-library-watch-section-desc = skwd-wall-এর বাইরে যোগ করা ফাইল শনাক্ত করে লাইব্রেরি হালনাগাদ রাখুন।
settings-library-watch-fallback-label = পোলিং ফলব্যাক
settings-library-watch-fallback-desc = শুধু সেই লাইব্রেরি ফোল্ডারগুলো পরীক্ষা করে, যেগুলো নেটিভ ফাইল পর্যবেক্ষণ দেখতে পারে না। পরিবর্তন ধরতে না পারা নেটওয়ার্ক বা FUSE মাউন্টের জন্য এটি চালু করুন, তারপর skwd-walld পুনরায় চালু করুন।
settings-library-watch-interval-label = পোলিং বিরতি
settings-library-watch-interval-desc = সীমিত পরীক্ষাগুলোর মধ্যে এত সেকেন্ড অপেক্ষা করে। কম মানে পরিবর্তন তাড়াতাড়ি ধরা পড়ে, কিন্তু ফাইলসিস্টেম বেশিবার পড়া হয়। এটি বদলানোর পর skwd-walld পুনরায় চালু করুন।
settings-library-watch-unknown-label = পর্যবেক্ষকের অবস্থা অনুপলব্ধ
settings-library-watch-unknown-desc = এই ডেমন লাইব্রেরি পর্যবেক্ষণের অবস্থা জানায় না। skwd-walld হালনাগাদ বা পুনরায় চালু করুন।
settings-library-watch-poll-failed-label = পোলিং একটি লাইব্রেরি ফোল্ডার পড়তে পারছে না
settings-library-watch-poll-failed-desc = কনফিগার করা প্রতিটি লাইব্রেরি ফোল্ডার মাউন্ট করা ও পঠনযোগ্য কি না পরীক্ষা করুন। পোলিং { $interval } সেকেন্ড পর আবার চেষ্টা করবে।
settings-library-watch-polling-label = পোলিং ফলব্যাক সক্রিয়
settings-library-watch-polling-desc = { $count ->
    [one] { $count }টি লাইব্রেরি ফোল্ডারে
   *[other] মোট { $count }টি লাইব্রেরি ফোল্ডারে
    } নেটিভ পর্যবেক্ষণ ব্যর্থ হয়েছে। প্রতি { $interval } সেকেন্ডে সর্বোচ্চ { $budget }টি এন্ট্রি পরীক্ষা করা হয়। সর্বশেষ সফল সমন্বয়: { $convergence }।
settings-library-watch-recovering-label = নেটিভ পর্যবেক্ষণ পুনরুদ্ধার হয়েছে
settings-library-watch-recovering-desc = নেটিভ পর্যবেক্ষক আবার সক্রিয়। লাইব্রেরি হালনাগাদ ঘোষণার আগে একটি পূর্ণ হস্তান্তর স্ক্যান এখনও চলছে।
settings-library-watch-unavailable-label = লাইব্রেরি পর্যবেক্ষণ অনুপলব্ধ
settings-library-watch-unavailable-desc = নেটিভ ফাইল পর্যবেক্ষণ ব্যর্থ হয়েছে এবং পোলিং ফলব্যাক বন্ধ। পোলিং ফলব্যাক চালু করুন, তারপর skwd-walld পুনরায় চালু করুন।
settings-library-watch-recovered-label = নেটিভ পর্যবেক্ষণ পুনঃস্থাপিত
settings-library-watch-recovered-desc = নেটিভ পর্যবেক্ষক ও তার হস্তান্তর স্ক্যান হালনাগাদ। সর্বশেষ সফল সমন্বয়: { $convergence }।
settings-library-watch-native-label = নেটিভ ফাইল পর্যবেক্ষণ
settings-library-watch-native-desc = প্রতিটি লাইব্রেরি ফোল্ডারে ফাইলসিস্টেম ইভেন্ট সক্রিয়। পোলিং নিষ্ক্রিয়।
settings-library-watch-convergence-never = এখনও সম্পন্ন হয়নি
settings-library-watch-convergence-seconds = { $value ->
    [one] { $value } সেকেন্ড আগে
   *[other] { $value } সেকেন্ড আগে
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] { $value } মিনিট আগে
   *[other] { $value } মিনিট আগে
    }
settings-library-watch-convergence-hours = { $value ->
    [one] { $value } ঘণ্টা আগে
   *[other] { $value } ঘণ্টা আগে
    }

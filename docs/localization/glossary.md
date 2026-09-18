# SilverSetup localization: English and Myanmar

The empty scaffold has no user-facing feature copy or localization runtime yet. Future features must provide feature-scoped `en` and `my` resources, persist the locale locally, use English as fallback, and format dates, counts, and relative times with `Intl`. Product names and version strings stay unchanged. The terms below are drafts pending native-speaker review.

| Concept | English | Myanmar UI term |
|---|---|---|
| Install | Install | ထည့်သွင်းရန် |
| Uninstall | Uninstall | ဖယ်ရှားရန် |
| Update | Update | အပ်ဒိတ်လုပ်ရန် |
| Installed | Installed | ထည့်သွင်းပြီး |
| Available version | Available version | ရရှိနိုင်သောဗားရှင်း |
| Developer tools | Developer tools | Developer ကိရိယာများ |
| Environment | Environment | အသုံးပြုသည့် ပတ်ဝန်းကျင် |
| System requirements | System requirements | စနစ်လိုအပ်ချက်များ |
| Activity | Activity | လုပ်ဆောင်ချက် |
| Settings | Settings | ဆက်တင်များ |
| Success | Success | အောင်မြင်သည် |
| Warning | Warning | သတိပေးချက် |
| Error | Error | အမှား |

Native-speaker review is required before declaring Myanmar copy release-ready. In particular, review hybrid technical terms such as **Developer ကိရိယာများ**, **Source control**, **runtime**, and **coding agent** in context. Keep established English technical terms when no clear professional Myanmar equivalent exists.

When features are implemented, put user-facing copy in both locales. Do not translate arbitrary backend error strings by guessing; use typed error codes. Select and review the localization and bundled-font dependencies at implementation time.

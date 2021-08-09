use poriborton::bijoy2000::unicode_to_bijoy;
use pretty_assertions::assert_eq;

#[test]
fn test_1() {
    assert_eq!(unicode_to_bijoy("ক্ক"), "°");
    assert_eq!(unicode_to_bijoy("আক্কেল"), "Av‡°j");
    assert_eq!(unicode_to_bijoy("টেক্কা"), "†U°v");
    assert_eq!(unicode_to_bijoy("ধাক্কা"), "av°v");
}

#[test]
fn test_2() {
    assert_eq!(unicode_to_bijoy("ক্ট"), "±");
    assert_eq!(unicode_to_bijoy("ডক্টর"), "W±i");
    assert_eq!(unicode_to_bijoy("অক্টোপাস"), "A‡±vcvm");
}

#[test]
fn test_3() {
    assert_eq!(unicode_to_bijoy("ক্ট্র"), "±ª");
    assert_eq!(unicode_to_bijoy("অক্ট্রয়"), "A±ªq");
}

#[test]
fn test_4() {
    assert_eq!(unicode_to_bijoy("ক্ত"), "³");
    assert_eq!(unicode_to_bijoy("রক্ত"), "i³");
    assert_eq!(unicode_to_bijoy("শক্ত"), "k³");
}

#[test]
fn test_5() {
    assert_eq!(unicode_to_bijoy("ক্ত্র"), "³«");
    assert_eq!(unicode_to_bijoy("বক্ত্র"), "e³«");
}

#[test]
fn test_6() {
    assert_eq!(unicode_to_bijoy("ক্ন"), "Kè");
    assert_eq!(unicode_to_bijoy("বাচক্নবী"), "evPKèex");
}

#[test]
fn test_7() {
    assert_eq!(unicode_to_bijoy("ক্ব"), "K¡");
    assert_eq!(unicode_to_bijoy("পক্ব"), "cK¡");
    assert_eq!(unicode_to_bijoy("ক্বণ"), "K¡Y");
}

#[test]
fn test_8() {
    assert_eq!(unicode_to_bijoy("ক্ম"), "´");
    assert_eq!(unicode_to_bijoy("রুক্মিণী"), "i“w´Yx");
}

#[test]
fn test_9() {
    assert_eq!(unicode_to_bijoy("ক্য"), "K¨");
    assert_eq!(unicode_to_bijoy("বাক্য"), "evK¨");
}

#[test]
fn test_10() {
    assert_eq!(unicode_to_bijoy("ক্র"), "µ");
    assert_eq!(unicode_to_bijoy("চক্র"), "Pµ");
    assert_eq!(unicode_to_bijoy("বক্র"), "eµ");
}

#[test]
fn test_11() {
    assert_eq!(unicode_to_bijoy("ক্ল"), "K¬");
    assert_eq!(unicode_to_bijoy("ক্লান্তি"), "K¬vwš—");
}

#[test]
fn test_12() {
    assert_eq!(unicode_to_bijoy("ক্ষ"), "¶");
    assert_eq!(unicode_to_bijoy("পক্ষ"), "c¶");
    assert_eq!(unicode_to_bijoy("ভক্ষক"), "f¶K");
    assert_eq!(unicode_to_bijoy("অপেক্ষা"), "A‡c¶v");
}

#[test]
fn test_13() {
    assert_eq!(unicode_to_bijoy("ক্ষ্ণ"), "¶è");
    assert_eq!(unicode_to_bijoy("তীক্ষ্ণ"), "Zx¶è");
}

#[test]
fn test_14() {
    assert_eq!(unicode_to_bijoy("ক্ষ্ব"), "¶¡");
    assert_eq!(unicode_to_bijoy("ইক্ষ্বাকু"), "B¶¡vKz");
}

#[test]
fn test_15() {
    assert_eq!(unicode_to_bijoy("ক্ষ্ম"), "²");
    assert_eq!(unicode_to_bijoy("লক্ষ্মী"), "j²x");
}

#[test]
fn test_16() {
    assert_eq!(unicode_to_bijoy("ক্ষ্ম্য"), "²¨");
    assert_eq!(unicode_to_bijoy("সৌক্ষ্ম্য"), "†mŠ²¨");
}

#[test]
fn test_17() {
    assert_eq!(unicode_to_bijoy("ক্ষ্য"), "¶¨");
    assert_eq!(unicode_to_bijoy("লক্ষ্য"), "j¶¨");
}

#[test]
fn test_18() {
    assert_eq!(unicode_to_bijoy("ক্স"), "·");
    assert_eq!(unicode_to_bijoy("বাক্স"), "ev·");
}

#[test]
fn test_19() {
    assert_eq!(unicode_to_bijoy("খ্য"), "L¨");
    assert_eq!(unicode_to_bijoy("সখ্য"), "mL¨");
    assert_eq!(unicode_to_bijoy("সাংখ্য"), "mvsL¨");
}

#[test]
fn test_20() {
    assert_eq!(unicode_to_bijoy("খ্র"), "Lª");
    assert_eq!(unicode_to_bijoy("খ্রিস্টান"), "wLª÷vb");
}

#[test]
fn test_21() {
    assert_eq!(unicode_to_bijoy("গ্‌ণ"), "M&Y");
    assert_eq!(unicode_to_bijoy("রুগ্‌ণ"), "i“M&Y");
}

#[test]
fn test_22() {
    assert_eq!(unicode_to_bijoy("গ্ধ"), "»");
    assert_eq!(unicode_to_bijoy("মুগ্ধ"), "gy»");
}

#[test]
fn test_23() {
    assert_eq!(unicode_to_bijoy("গ্ধ্য"), "»¨");
    assert_eq!(unicode_to_bijoy("বৈদগ্ধ্য"), "ˆe`»¨");
}

#[test]
fn test_24() {
    assert_eq!(unicode_to_bijoy("গ্ধ্র"), "»ª");
    assert_eq!(unicode_to_bijoy("দোগ্ধ্রী"), "†`v»ªx");
}

#[test]
fn test_25() {
    assert_eq!(unicode_to_bijoy("গ্ন"), "Mœ");
    assert_eq!(unicode_to_bijoy("ভগ্ন"), "fMœ");
    assert_eq!(unicode_to_bijoy("অগ্নি"), "AwMœ");
}

#[test]
fn test_26() {
    assert_eq!(unicode_to_bijoy("গ্ন্য"), "Mœ¨");
    assert_eq!(unicode_to_bijoy("অগ্ন্যাস্ত্র"), "AMœ¨v¯¿");
    assert_eq!(unicode_to_bijoy("অগ্ন্যুৎপাত"), "AMœ¨yrcvZ");
    assert_eq!(unicode_to_bijoy("অগ্ন্যাশয়"), "AMœ¨vkq");
}

#[test]
fn test_27() {
    assert_eq!(unicode_to_bijoy("গ্ব"), "M¦");
    assert_eq!(unicode_to_bijoy("দিগ্বিজয়ী"), "w`wM¦Rqx");
}

#[test]
fn test_28() {
    assert_eq!(unicode_to_bijoy("গ্ম"), "M¥");
    assert_eq!(unicode_to_bijoy("যুগ্ম"), "hyM¥");
}

#[test]
fn test_29() {
    assert_eq!(unicode_to_bijoy("গ্য"), "M¨");
    assert_eq!(unicode_to_bijoy("ভাগ্য"), "fvM¨");
}

#[test]
fn test_30() {
    assert_eq!(unicode_to_bijoy("গ্র"), "MÖ");
    assert_eq!(unicode_to_bijoy("গ্রাম"), "MÖvg");
}

#[test]
fn test_31() {
    assert_eq!(unicode_to_bijoy("গ্র্য"), "MÖ¨");
    assert_eq!(unicode_to_bijoy("ঐকাগ্র্য"), "HKvMÖ¨");
    assert_eq!(unicode_to_bijoy("সামগ্র্য"), "mvgMÖ¨");
    assert_eq!(unicode_to_bijoy("গ্র্যাজুয়েট"), "MÖ¨vRz‡qU");
}

#[test]
fn test_32() {
    assert_eq!(unicode_to_bijoy("গ্ল"), "M−");
    assert_eq!(unicode_to_bijoy("গ্লানি"), "M−vwb");
}

#[test]
fn test_33() {
    assert_eq!(unicode_to_bijoy("ঘ্ন"), "Nœ");
    assert_eq!(unicode_to_bijoy("কৃতঘ্ন"), "K…ZNœ");
    assert_eq!(unicode_to_bijoy("শত্রুঘ্ন"), "kÎ“Nœ");
}

#[test]
fn test_34() {
    assert_eq!(unicode_to_bijoy("ঘ্য"), "N¨");
    assert_eq!(unicode_to_bijoy("অশ্লাঘ্য"), "Ak−vN¨");
}

#[test]
fn test_35() {
    assert_eq!(unicode_to_bijoy("ঘ্র"), "Nª");
    assert_eq!(unicode_to_bijoy("ঘ্রাণ"), "NªvY");
}

#[test]
fn test_36() {
    assert_eq!(unicode_to_bijoy("ঙ্ক"), "¼");
    assert_eq!(unicode_to_bijoy("অঙ্ক"), "A¼");
    assert_eq!(unicode_to_bijoy("টঙ্কা"), "U¼v");
    assert_eq!(unicode_to_bijoy("শশাঙ্ক"), "kkv¼");
}

#[test]
fn test_37() {
    assert_eq!(unicode_to_bijoy("ঙ্‌ক্ত"), "O&³");
    assert_eq!(unicode_to_bijoy("পঙ্‌ক্তি"), "cO&w³");
}

#[test]
fn test_38() {
    assert_eq!(unicode_to_bijoy("ঙ্ক্য"), "¼¨");
    assert_eq!(unicode_to_bijoy("অঙ্ক্য"), "A¼¨");
}

#[test]
fn test_39() {
    assert_eq!(unicode_to_bijoy("ঙ্ক্ষ"), "•¶");
    assert_eq!(unicode_to_bijoy("আকাঙ্ক্ষা"), "AvKv•¶v");
}

#[test]
fn test_40() {
    assert_eq!(unicode_to_bijoy("ঙ্খ"), "•L");
    assert_eq!(unicode_to_bijoy("শঙ্খ"), "k•L");
}

#[test]
fn test_41() {
    assert_eq!(unicode_to_bijoy("ঙ্খ্য"), "•L¨");
    assert_eq!(unicode_to_bijoy("সাঙ্খ্যমান"), "mv•L¨gvb");
}

#[test]
fn test_42() {
    assert_eq!(unicode_to_bijoy("ঙ্গ"), "½");
    assert_eq!(unicode_to_bijoy("অঙ্গ"), "A½");
    assert_eq!(unicode_to_bijoy("সঙ্গী"), "m½x");
}

#[test]
fn test_43() {
    assert_eq!(unicode_to_bijoy("ঙ্গ্য"), "½¨");
    assert_eq!(unicode_to_bijoy("ব্যঙ্গ্যার্থ"), "e¨½¨v_©");
    assert_eq!(unicode_to_bijoy("ব্যঙ্গ্যোক্তি"), "e¨‡½¨vw³");
}

#[test]
fn test_44() {
    assert_eq!(unicode_to_bijoy("ঙ্ঘ"), "•N");
    assert_eq!(unicode_to_bijoy("সঙ্ঘ"), "m•N");
}

#[test]
fn test_45() {
    assert_eq!(unicode_to_bijoy("ঙ্ঘ্য"), "•N¨");
    assert_eq!(unicode_to_bijoy("দুর্লঙ্ঘ্য"), "`yj©•N¨");
}

#[test]
fn test_46() {
    assert_eq!(unicode_to_bijoy("ঙ্ঘ্র"), "•Nª");
    assert_eq!(unicode_to_bijoy("অঙ্ঘ্রি"), "Aw•Nª");
}

#[test]
fn test_47() {
    assert_eq!(unicode_to_bijoy("ঙ্ম"), "•g");
    assert_eq!(unicode_to_bijoy("বাঙ্ময়"), "ev•gq");
}

#[test]
fn test_48() {
    assert_eq!(unicode_to_bijoy("চ্চ"), "”P");
    assert_eq!(unicode_to_bijoy("বাচ্চা"), "ev”Pv");
}

#[test]
fn test_49() {
    assert_eq!(unicode_to_bijoy("চ্ছ"), "”Q");
    assert_eq!(unicode_to_bijoy("ইচ্ছা"), "B”Qv");
}

#[test]
fn test_50() {
    assert_eq!(unicode_to_bijoy("চ্ছ্ব"), "”Q¡");
    assert_eq!(unicode_to_bijoy("জলোচ্ছ্বাস"), "R‡jv”Q¡vm");
}

#[test]
fn test_51() {
    assert_eq!(unicode_to_bijoy("চ্ছ্র"), "”Qª");
    assert_eq!(unicode_to_bijoy("উচ্ছ্রায়"), "D”Qªvq");
}

#[test]
fn test_52() {
    assert_eq!(unicode_to_bijoy("চ্ঞ"), "”T");
    assert_eq!(unicode_to_bijoy("যাচ্ঞা"), "hv”Tv");
}

#[test]
fn test_53() {
    assert_eq!(unicode_to_bijoy("চ্ব"), "”¡");
    assert_eq!(unicode_to_bijoy("চ্বী"), "”¡x");
}

#[test]
fn test_54() {
    assert_eq!(unicode_to_bijoy("চ্য"), "P¨");
    assert_eq!(unicode_to_bijoy("প্রাচ্য"), "cÖvP¨");
}

#[test]
fn test_55() {
    assert_eq!(unicode_to_bijoy("জ্জ"), "¾");
    assert_eq!(unicode_to_bijoy("বিপজ্জনক"), "wec¾bK");
}

#[test]
fn test_56() {
    assert_eq!(unicode_to_bijoy("জ্জ্ব"), "¾¡");
    assert_eq!(unicode_to_bijoy("উজ্জ্বল"), "D¾¡j");
}

#[test]
fn test_57() {
    assert_eq!(unicode_to_bijoy("জ্ঝ"), "À");
    assert_eq!(unicode_to_bijoy("কুজ্ঝটিকা"), "KzÀwUKv");
}

#[test]
fn test_58() {
    assert_eq!(unicode_to_bijoy("জ্ঞ"), "Á");
    assert_eq!(unicode_to_bijoy("জ্ঞান"), "Ávb");
    assert_eq!(unicode_to_bijoy("সজ্ঞাত"), "mÁvZ");
}

#[test]
fn test_59() {
    assert_eq!(unicode_to_bijoy("জ্ব"), "R¡");
    assert_eq!(unicode_to_bijoy("জ্বর"), "R¡i");
}

#[test]
fn test_60() {
    assert_eq!(unicode_to_bijoy("জ্য"), "R¨");
    assert_eq!(unicode_to_bijoy("রাজ্য"), "ivR¨");
}

#[test]
fn test_61() {
    assert_eq!(unicode_to_bijoy("জ্র"), "Rª");
    assert_eq!(unicode_to_bijoy("বজ্র"), "eRª");
}

#[test]
fn test_62() {
    assert_eq!(unicode_to_bijoy("ঞ্চ"), "Â");
    assert_eq!(unicode_to_bijoy("অঞ্চল"), "AÂj");
    assert_eq!(unicode_to_bijoy("সঞ্চয়"), "mÂq");
}

#[test]
fn test_63() {
    assert_eq!(unicode_to_bijoy("ঞ্ছ"), "Ã");
    assert_eq!(unicode_to_bijoy("লাঞ্ছনা"), "jvÃbv");
    assert_eq!(unicode_to_bijoy("বাঞ্ছা"), "evÃv");
}

#[test]
fn test_64() {
    assert_eq!(unicode_to_bijoy("ঞ্জ"), "Ä");
    assert_eq!(unicode_to_bijoy("কুঞ্জ"), "KzÄ");
    assert_eq!(unicode_to_bijoy("গুঞ্জন"), "¸Äb");
}

#[test]
fn test_65() {
    assert_eq!(unicode_to_bijoy("ঞ্ঝ"), "Å");
    assert_eq!(unicode_to_bijoy("ঝঞ্ঝা"), "SÅv");
}

#[test]
fn test_66() {
    assert_eq!(unicode_to_bijoy("ট্ট"), "Æ");
    assert_eq!(unicode_to_bijoy("চট্টগ্রাম"), "PÆMÖvg");
    assert_eq!(unicode_to_bijoy("ছোট্ট"), "†QvÆ");
}

#[test]
fn test_67() {
    assert_eq!(unicode_to_bijoy("ট্ব"), "U¡");
    assert_eq!(unicode_to_bijoy("খট্বা"), "LU¡v");
}

#[test]
fn test_68() {
    assert_eq!(unicode_to_bijoy("ট্ম"), "U¥");
    assert_eq!(unicode_to_bijoy("কুট্মল"), "KzU¥j");
}

#[test]
fn test_69() {
    assert_eq!(unicode_to_bijoy("ট্য"), "U¨");
    assert_eq!(unicode_to_bijoy("নাট্য"), "bvU¨");
}

#[test]
fn test_70() {
    assert_eq!(unicode_to_bijoy("ট্র"), "Uª");
    assert_eq!(unicode_to_bijoy("ট্রেন"), "†Uªb");
}

#[test]
fn test_71() {
    assert_eq!(unicode_to_bijoy("ড্ড"), "Ç");
    assert_eq!(unicode_to_bijoy("আড্ডা"), "AvÇv");
    assert_eq!(unicode_to_bijoy("উড্ডয়ন"), "DÇqb");
}

#[test]
fn test_72() {
    assert_eq!(unicode_to_bijoy("ড্ব"), "W¡");
    assert_eq!(unicode_to_bijoy("অন্ড্বান"), "AÛ¡vb");
}

#[test]
fn test_73() {
    assert_eq!(unicode_to_bijoy("ড্ম"), "W¥");
    assert_eq!(unicode_to_bijoy("কুড্মল"), "KzW¥j");
}

#[test]
fn test_74() {
    assert_eq!(unicode_to_bijoy("ড্য"), "W¨");
    assert_eq!(unicode_to_bijoy("জাড্য"), "RvW¨");
}

#[test]
fn test_75() {
    assert_eq!(unicode_to_bijoy("ড্র"), "Wª");
    assert_eq!(unicode_to_bijoy("ড্রাইভার"), "WªvBfvi");
    assert_eq!(unicode_to_bijoy("ড্রাম"), "Wªvg");
}

#[test]
fn test_76() {
    assert_eq!(unicode_to_bijoy("ড়্গ"), "ÿ");
    assert_eq!(unicode_to_bijoy("খড়্গ"), "Lÿ");
}

#[test]
fn test_77() {
    assert_eq!(unicode_to_bijoy("ঢ্য"), "X¨");
    assert_eq!(unicode_to_bijoy("ধনাঢ্য"), "abvX¨");
}

#[test]
fn test_78() {
    assert_eq!(unicode_to_bijoy("ঢ্র"), "Xª");
    assert_eq!(unicode_to_bijoy("মেঢ্র"), "†gXª");
}

#[test]
fn test_79() {
    assert_eq!(unicode_to_bijoy("ণ্ট"), "È");
    assert_eq!(unicode_to_bijoy("ঘণ্টা"), "NÈv");
    assert_eq!(unicode_to_bijoy("নির্ঘণ্ট"), "wbN©È");
}

#[test]
fn test_80() {
    assert_eq!(unicode_to_bijoy("ণ্ঠ"), "É");
    assert_eq!(unicode_to_bijoy("কণ্ঠ"), "KÉ");
}

#[test]
fn test_81() {
    assert_eq!(unicode_to_bijoy("ণ্ঠ্য"), "É¨");
    assert_eq!(unicode_to_bijoy("কণ্ঠ্য"), "KÉ¨");
}

#[test]
fn test_82() {
    assert_eq!(unicode_to_bijoy("ণ্ড"), "Ê");
    assert_eq!(unicode_to_bijoy("গণ্ডগোল"), "MÊ‡Mvj");
    assert_eq!(unicode_to_bijoy("কুণ্ড"), "KzÊ");
}

#[test]
fn test_83() {
    assert_eq!(unicode_to_bijoy("ণ্ড্য"), "Ê¨");
    assert_eq!(unicode_to_bijoy("পাণ্ড্য"), "cvÊ¨");
}

#[test]
fn test_84() {
    assert_eq!(unicode_to_bijoy("ণ্ড্র"), "Êª");
    assert_eq!(unicode_to_bijoy("পুণ্ড্র"), "cyÊª");
}

#[test]
fn test_85() {
    assert_eq!(unicode_to_bijoy("ণ্ঢ"), "YX");
    assert_eq!(unicode_to_bijoy("ষণ্ঢ"), "lYX");
}

#[test]
fn test_86() {
    assert_eq!(unicode_to_bijoy("ণ্ণ"), "Yè");
    assert_eq!(unicode_to_bijoy("বিষণ্ণ"), "welYè");
}

#[test]
fn test_87() {
    assert_eq!(unicode_to_bijoy("ণ্ব"), "Y¡");
    assert_eq!(unicode_to_bijoy("স্থাণ্বীশ্বর"), "¯’vY¡xk¦i");
}

#[test]
fn test_88() {
    assert_eq!(unicode_to_bijoy("ণ্ম"), "Y¥");
    assert_eq!(unicode_to_bijoy("চিণ্ময়"), "wPY¥q");
    assert_eq!(unicode_to_bijoy("মৃণ্ময়ী"), "g„Y¥qx");
}

#[test]
fn test_89() {
    assert_eq!(unicode_to_bijoy("ণ্য"), "Y¨");
    assert_eq!(unicode_to_bijoy("পূণ্য"), "c~Y¨");
}

#[test]
fn test_90() {
    assert_eq!(unicode_to_bijoy("ৎক"), "rK");
    assert_eq!(unicode_to_bijoy("উৎকর্ষ"), "DrKl©");
}

#[test]
fn test_91() {
    assert_eq!(unicode_to_bijoy("ৎখ"), "rL");
    assert_eq!(unicode_to_bijoy("উৎখাত"), "DrLvZ");
}

#[test]
fn test_92() {
    assert_eq!(unicode_to_bijoy("ত্ত"), "Ë");
    assert_eq!(unicode_to_bijoy("উত্তর"), "DËi");
}

#[test]
fn test_93() {
    assert_eq!(unicode_to_bijoy("ত্ত্ব"), "Ë¡");
    assert_eq!(unicode_to_bijoy("সত্ত্ব"), "mË¡");
}

#[test]
fn test_94() {
    assert_eq!(unicode_to_bijoy("ত্ত্য"), "Ë¨");
    assert_eq!(unicode_to_bijoy("উত্ত্যক্ত"), "DË¨³");
}

#[test]
fn test_95() {
    assert_eq!(unicode_to_bijoy("ত্থ"), "Ì");
    assert_eq!(unicode_to_bijoy("অশ্বত্থ"), "Ak¦Ì");
}

#[test]
fn test_96() {
    assert_eq!(unicode_to_bijoy("ত্ন"), "Zœ");
    assert_eq!(unicode_to_bijoy("যত্ন"), "hZœ");
    assert_eq!(unicode_to_bijoy("রত্নাকর"), "iZœvKi");
}

#[test]
fn test_97() {
    assert_eq!(unicode_to_bijoy("ৎপ"), "rc");
    assert_eq!(unicode_to_bijoy("উৎপল"), "Drcj");
}

#[test]
fn test_98() {
    assert_eq!(unicode_to_bijoy("ত্ব"), "Z¡");
    assert_eq!(unicode_to_bijoy("রাজত্ব"), "ivRZ¡");
}

#[test]
fn test_99() {
    assert_eq!(unicode_to_bijoy("ত্ম"), "Í");
    assert_eq!(unicode_to_bijoy("আত্মা"), "AvÍv");
}

#[test]
fn test_100() {
    assert_eq!(unicode_to_bijoy("ত্ম্য"), "Í¨");
    assert_eq!(unicode_to_bijoy("দৌরাত্ম্য"), "†`ŠivÍ¨");
}

#[test]
fn test_101() {
    assert_eq!(unicode_to_bijoy("ত্য"), "Z¨");
    assert_eq!(unicode_to_bijoy("সত্য"), "mZ¨");
}

#[test]
fn test_102() {
    assert_eq!(unicode_to_bijoy("ত্র"), "Î");
    assert_eq!(unicode_to_bijoy("ত্রিশ"), "wÎk");
    assert_eq!(unicode_to_bijoy("ত্রাণ"), "ÎvY");
}

#[test]
fn test_103() {
    assert_eq!(unicode_to_bijoy("ত্র্য"), "Î¨");
    assert_eq!(unicode_to_bijoy("বৈচিত্র্য"), "ˆewPÎ¨");
}

#[test]
fn test_104() {
    assert_eq!(unicode_to_bijoy("ৎল"), "rj");
    assert_eq!(unicode_to_bijoy("কাৎলা"), "Kvrjv");
}

#[test]
fn test_105() {
    assert_eq!(unicode_to_bijoy("ৎস"), "rm");
    assert_eq!(unicode_to_bijoy("বৎসর"), "ermi");
    assert_eq!(unicode_to_bijoy("উৎসব"), "Drme");
}

#[test]
fn test_106() {
    assert_eq!(unicode_to_bijoy("থ্ব"), "_¡");
    assert_eq!(unicode_to_bijoy("পৃথ্বী"), "c„_¡x");
}

#[test]
fn test_107() {
    assert_eq!(unicode_to_bijoy("থ্য"), "_¨");
    assert_eq!(unicode_to_bijoy("পথ্য"), "c_¨");
}

#[test]
fn test_108() {
    assert_eq!(unicode_to_bijoy("থ্র"), "_ª");
    assert_eq!(unicode_to_bijoy("থ্রি"), "w_ª");
}

#[test]
fn test_109() {
    assert_eq!(unicode_to_bijoy("দ্গ"), "˜M");
    assert_eq!(unicode_to_bijoy("উদ্গম"), "D˜Mg");
}

#[test]
fn test_110() {
    assert_eq!(unicode_to_bijoy("দ্ঘ"), "™N");
    assert_eq!(unicode_to_bijoy("উদ্ঘাটন"), "D™NvUb");
}

#[test]
fn test_111() {
    assert_eq!(unicode_to_bijoy("দ্দ"), "Ï");
    assert_eq!(unicode_to_bijoy("উদ্দেশ্য"), "D‡Ïk¨");
}

#[test]
fn test_112() {
    assert_eq!(unicode_to_bijoy("দ্দ্ব"), "Ï¡");
    assert_eq!(unicode_to_bijoy("তদ্দ্বারা"), "ZÏ¡viv");
}

#[test]
fn test_113() {
    assert_eq!(unicode_to_bijoy("দ্ধ"), "×");
    assert_eq!(unicode_to_bijoy("রুদ্ধ"), "i“×");
    assert_eq!(unicode_to_bijoy("বদ্ধ"), "e×");
}

#[test]
fn test_114() {
    assert_eq!(unicode_to_bijoy("দ্ব"), "Ø");
    assert_eq!(unicode_to_bijoy("বিদ্বান"), "weØvb");
}

#[test]
fn test_115() {
    assert_eq!(unicode_to_bijoy("দ্ভ"), "™¢");
    assert_eq!(unicode_to_bijoy("অদ্ভুত"), "A™¢zZ");
}

#[test]
fn test_116() {
    assert_eq!(unicode_to_bijoy("দ্ভ্র"), "™£");
    assert_eq!(unicode_to_bijoy("উদ্ভ্রান্ত"), "D™£vš—");
}

#[test]
fn test_117() {
    assert_eq!(unicode_to_bijoy("দ্ম"), "Ù");
    assert_eq!(unicode_to_bijoy("ছদ্ম"), "QÙ");
    assert_eq!(unicode_to_bijoy("পদ্ম"), "cÙ");
}

#[test]
fn test_118() {
    assert_eq!(unicode_to_bijoy("দ্য"), "`¨");
    assert_eq!(unicode_to_bijoy("বাদ্য"), "ev`¨");
    assert_eq!(unicode_to_bijoy("পদ্য"), "c`¨");
}

#[test]
fn test_119() {
    assert_eq!(unicode_to_bijoy("দ্র"), "`ª");
    assert_eq!(unicode_to_bijoy("রুদ্র"), "i“`ª");
    assert_eq!(unicode_to_bijoy("নিদ্রিত"), "wbw`ªZ");
}

#[test]
fn test_120() {
    assert_eq!(unicode_to_bijoy("দ্র্য"), "`ª¨");
    assert_eq!(unicode_to_bijoy("দারিদ্র্য"), "`vwi`ª¨");
}

#[test]
fn test_121() {
    assert_eq!(unicode_to_bijoy("ধ্ন"), "aœ");
    assert_eq!(unicode_to_bijoy("অর্থগৃধ্নু"), "A_©M„aœy");
}

#[test]
fn test_122() {
    assert_eq!(unicode_to_bijoy("ধ্ব"), "aŸ");
    assert_eq!(unicode_to_bijoy("ধ্বনি"), "aŸwb");
}

#[test]
fn test_123() {
    assert_eq!(unicode_to_bijoy("ধ্ম"), "a¥");
    assert_eq!(unicode_to_bijoy("উদরাধ্মান"), "D`iva¥vb");
}

#[test]
fn test_124() {
    assert_eq!(unicode_to_bijoy("ধ্য"), "a¨");
    assert_eq!(unicode_to_bijoy("আরাধ্য"), "Aviva¨");
}

#[test]
fn test_125() {
    assert_eq!(unicode_to_bijoy("ধ্র"), "aª");
    assert_eq!(unicode_to_bijoy("ধ্রুব"), "aª“e");
}

#[test]
fn test_126() {
    assert_eq!(unicode_to_bijoy("ন্ট"), "›U");
    assert_eq!(unicode_to_bijoy("প্যান্ট"), "c¨v›U");
}

#[test]
fn test_127() {
    assert_eq!(unicode_to_bijoy("ন্ট্র"), "›Uª");
    assert_eq!(unicode_to_bijoy("কন্ট্রোল"), "K‡›Uªvj");
}

#[test]
fn test_128() {
    assert_eq!(unicode_to_bijoy("ন্ঠ"), "Ú");
    assert_eq!(unicode_to_bijoy("লন্ঠন"), "jÚb");
}

#[test]
fn test_129() {
    assert_eq!(unicode_to_bijoy("ন্ড"), "Û");
    assert_eq!(unicode_to_bijoy("গন্ডার"), "MÛvi");
    assert_eq!(unicode_to_bijoy("পাউন্ড"), "cvDÛ");
}

#[test]
fn test_130() {
    assert_eq!(unicode_to_bijoy("ন্ড্র"), "Ûª");
    assert_eq!(unicode_to_bijoy("হান্ড্রেড"), "nv‡ÛªW");
}

#[test]
fn test_131() {
    assert_eq!(unicode_to_bijoy("ন্ত"), "š—");
    assert_eq!(unicode_to_bijoy("জীবন্ত"), "Rxeš—");
    assert_eq!(unicode_to_bijoy("গন্তব্য"), "Mš—e¨");
}

#[test]
fn test_132() {
    assert_eq!(unicode_to_bijoy("ন্ত্ব"), "š—¡");
    assert_eq!(unicode_to_bijoy("সান্ত্বনা"), "mvš—¡bv");
}

#[test]
fn test_133() {
    assert_eq!(unicode_to_bijoy("ন্ত্য"), "š—¨");
    assert_eq!(unicode_to_bijoy("অন্ত্য"), "Aš—¨");
}

#[test]
fn test_134() {
    assert_eq!(unicode_to_bijoy("ন্ত্র"), "š¿");
    assert_eq!(unicode_to_bijoy("মন্ত্র"), "gš¿");
    assert_eq!(unicode_to_bijoy("যন্ত্র"), "hš¿");
}

#[test]
fn test_135() {
    assert_eq!(unicode_to_bijoy("ন্ত্র্য"), "š¿¨");
    assert_eq!(unicode_to_bijoy("স্বাতন্ত্র্য"), "¯^vZš¿¨");
}

#[test]
fn test_136() {
    assert_eq!(unicode_to_bijoy("ন্থ"), "š’");
    assert_eq!(unicode_to_bijoy("গ্রন্থ"), "MÖš’");
    assert_eq!(unicode_to_bijoy("পান্থ"), "cvš’");
}

#[test]
fn test_137() {
    assert_eq!(unicode_to_bijoy("ন্থ্র"), "š’ª");
    assert_eq!(unicode_to_bijoy("অ্যান্থ্রাক্স"), "A¨vš’ªv·");
}

#[test]
fn test_138() {
    assert_eq!(unicode_to_bijoy("ন্দ"), "›`");
    assert_eq!(unicode_to_bijoy("ছন্দ"), "Q›`");
}

#[test]
fn test_139() {
    assert_eq!(unicode_to_bijoy("ন্দ্য"), "›`¨");
    assert_eq!(unicode_to_bijoy("অনিন্দ্য"), "Awb›`¨");
}

#[test]
fn test_140() {
    assert_eq!(unicode_to_bijoy("ন্দ্ব"), "›Ø");
    assert_eq!(unicode_to_bijoy("দ্বন্দ্ব"), "Ø›Ø");
}

#[test]
fn test_141() {
    assert_eq!(unicode_to_bijoy("ন্দ্র"), "›`ª");
    assert_eq!(unicode_to_bijoy("কেন্দ্র"), "†K›`ª");
}

#[test]
fn test_142() {
    assert_eq!(unicode_to_bijoy("ন্ধ"), "Ü");
    assert_eq!(unicode_to_bijoy("অন্ধ"), "AÜ");
}

#[test]
fn test_143() {
    assert_eq!(unicode_to_bijoy("ন্ধ্য"), "Ü¨");
    assert_eq!(unicode_to_bijoy("বিন্ধ্য"), "weÜ¨");
}

#[test]
fn test_144() {
    assert_eq!(unicode_to_bijoy("ন্ধ্র"), "Üª");
    assert_eq!(unicode_to_bijoy("রন্ধ্র"), "iÜª");
}

#[test]
fn test_145() {
    assert_eq!(unicode_to_bijoy("ন্ন"), "bœ");
    assert_eq!(unicode_to_bijoy("নবান্ন"), "bevbœ");
}

#[test]
fn test_146() {
    assert_eq!(unicode_to_bijoy("ন্ব"), "š^");
    assert_eq!(unicode_to_bijoy("ধন্বন্তরি"), "aš^š—wi");
}

#[test]
fn test_147() {
    assert_eq!(unicode_to_bijoy("ন্ম"), "b¥");
    assert_eq!(unicode_to_bijoy("চিন্ময়"), "wPb¥q");
}

#[test]
fn test_148() {
    assert_eq!(unicode_to_bijoy("ন্য"), "b¨");
    assert_eq!(unicode_to_bijoy("ধন্য"), "ab¨");
    assert_eq!(unicode_to_bijoy("ধান্য"), "avb¨");
}

#[test]
fn test_149() {
    assert_eq!(unicode_to_bijoy("প্ট"), "Þ");
    assert_eq!(unicode_to_bijoy("পাটি-সাপ্টা"), "cvwU-mvÞv");
    assert_eq!(unicode_to_bijoy("ক্যাপ্টেন"), "K¨v‡Þb");
}

#[test]
fn test_150() {
    assert_eq!(unicode_to_bijoy("প্ত"), "ß");
    assert_eq!(unicode_to_bijoy("সুপ্ত"), "myß");
}

#[test]
fn test_151() {
    assert_eq!(unicode_to_bijoy("প্ন"), "cœ");
    assert_eq!(unicode_to_bijoy("স্বপ্ন"), "¯^cœ");
}

#[test]
fn test_152() {
    assert_eq!(unicode_to_bijoy("প্প"), "à");
    assert_eq!(unicode_to_bijoy("ধাপ্পা"), "avàv");
}

#[test]
fn test_153() {
    assert_eq!(unicode_to_bijoy("প্য"), "c¨");
    assert_eq!(unicode_to_bijoy("প্রাপ্য"), "cÖvc¨");
}

#[test]
fn test_154() {
    assert_eq!(unicode_to_bijoy("প্র"), "cÖ");
    assert_eq!(unicode_to_bijoy("ক্ষিপ্র"), "w¶cÖ");
}

#[test]
fn test_155() {
    assert_eq!(unicode_to_bijoy("প্র্য"), "cÖ¨");
    assert_eq!(unicode_to_bijoy("প্র্যাকটিস"), "cÖ¨vKwUm");
}

#[test]
fn test_156() {
    assert_eq!(unicode_to_bijoy("প্ল"), "c­");
    assert_eq!(unicode_to_bijoy("আপ্লুত"), "Avc­“Z");
}

#[test]
fn test_157() {
    assert_eq!(unicode_to_bijoy("প্স"), "á");
    assert_eq!(unicode_to_bijoy("লিপ্সা"), "wjáv");
}

#[test]
fn test_158() {
    assert_eq!(unicode_to_bijoy("ফ্র"), "d«");
    assert_eq!(unicode_to_bijoy("ফ্রক"), "d«K");
    assert_eq!(unicode_to_bijoy("ফ্রিজ"), "wd«R");
    assert_eq!(unicode_to_bijoy("আফ্রিকা"), "Avwd«Kv");
    assert_eq!(unicode_to_bijoy("রেফ্রিজারেটর"), "†iwd«Rv‡iUi");
}

#[test]
fn test_159() {
    assert_eq!(unicode_to_bijoy("ফ্ল"), "d¬");
    assert_eq!(unicode_to_bijoy("ফ্লেভার"), "†d¬fvi");
}

#[test]
fn test_160() {
    assert_eq!(unicode_to_bijoy("ব্জ"), "â");
    assert_eq!(unicode_to_bijoy("ন্যুব্জ"), "b¨yâ");
}

#[test]
fn test_161() {
    assert_eq!(unicode_to_bijoy("ব্দ"), "ã");
    assert_eq!(unicode_to_bijoy("জব্দ"), "Rã");
    assert_eq!(unicode_to_bijoy("শব্দ"), "kã");
}

#[test]
fn test_162() {
    assert_eq!(unicode_to_bijoy("ব্ধ"), "ä");
    assert_eq!(unicode_to_bijoy("লব্ধ"), "jä");
}

#[test]
fn test_163() {
    assert_eq!(unicode_to_bijoy("ব্ব"), "eŸ");
    assert_eq!(unicode_to_bijoy("ডাব্বা"), "WveŸv");
}

#[test]
fn test_164() {
    assert_eq!(unicode_to_bijoy("ব্য"), "e¨");
    assert_eq!(unicode_to_bijoy("দাতব্য"), "`vZe¨");
    assert_eq!(unicode_to_bijoy("কর্তব্য"), "KZ©e¨");
}

#[test]
fn test_165() {
    assert_eq!(unicode_to_bijoy("ব্র"), "eª");
    assert_eq!(unicode_to_bijoy("ব্রাহ্মণ"), "eªvþY");
}

#[test]
fn test_166() {
    assert_eq!(unicode_to_bijoy("ব্ল"), "e­");
    assert_eq!(unicode_to_bijoy("ব্লাউজ"), "e­vDR");
}

#[test]
fn test_167() {
    assert_eq!(unicode_to_bijoy("ভ্ব"), "f¡");
    assert_eq!(unicode_to_bijoy("ভ্বা"), "f¡v");
}

#[test]
fn test_168() {
    assert_eq!(unicode_to_bijoy("ভ্য"), "f¨");
    assert_eq!(unicode_to_bijoy("সভ্য"), "mf¨");
}

#[test]
fn test_169() {
    assert_eq!(unicode_to_bijoy("ভ্র"), "å");
    assert_eq!(unicode_to_bijoy("শুভ্র"), "ïå");
    assert_eq!(unicode_to_bijoy("অভ্র"), "Aå");
}

#[test]
fn test_170() {
    assert_eq!(unicode_to_bijoy("ভ্ল"), "f¬");
    assert_eq!(unicode_to_bijoy("ভ্লাদিমি"), "f¬vw`wg");
}

#[test]
fn test_171() {
    assert_eq!(unicode_to_bijoy("ম্ন"), "æ");
    assert_eq!(unicode_to_bijoy("নিম্ন"), "wbæ");
}

#[test]
fn test_172() {
    assert_eq!(unicode_to_bijoy("ম্প"), "¤c");
    assert_eq!(unicode_to_bijoy("কম্প"), "K¤c");
    assert_eq!(unicode_to_bijoy("শম্পা"), "k¤cv");
}

#[test]
fn test_173() {
    assert_eq!(unicode_to_bijoy("ম্প্র"), "¤cÖ");
    assert_eq!(unicode_to_bijoy("সম্প্রতি"), "m¤cÖwZ");
}

#[test]
fn test_174() {
    assert_eq!(unicode_to_bijoy("ম্ফ"), "ç");
    assert_eq!(unicode_to_bijoy("লম্ফ"), "jç");
}

#[test]
fn test_175() {
    assert_eq!(unicode_to_bijoy("ম্ব"), "¤^");
    assert_eq!(unicode_to_bijoy("প্রতিবিম্ব"), "cÖwZwe¤^");
    assert_eq!(unicode_to_bijoy("অম্বর"), "A¤^i");
}

#[test]
fn test_176() {
    assert_eq!(unicode_to_bijoy("ম্ব্র"), "¤^ª");
    assert_eq!(unicode_to_bijoy("মেম্ব্রেন"), "†g‡¤^ªb");
}

#[test]
fn test_177() {
    assert_eq!(unicode_to_bijoy("ম্ভ"), "¤¢");
    assert_eq!(unicode_to_bijoy("দম্ভ"), "`¤¢");
}

#[test]
fn test_178() {
    assert_eq!(unicode_to_bijoy("ম্ভ্র"), "¤£");
    assert_eq!(unicode_to_bijoy("সম্ভ্রম"), "m¤£g");
}

#[test]
fn test_179() {
    assert_eq!(unicode_to_bijoy("ম্ম"), "¤§");
    assert_eq!(unicode_to_bijoy("সম্মান"), "m¤§vb");
}

#[test]
fn test_180() {
    assert_eq!(unicode_to_bijoy("ম্য"), "g¨");
    assert_eq!(unicode_to_bijoy("গ্রাম্য"), "MÖvg¨");
}

#[test]
fn test_181() {
    assert_eq!(unicode_to_bijoy("ম্র"), "gª");
    assert_eq!(unicode_to_bijoy("নম্র"), "bgª");
}

#[test]
fn test_182() {
    assert_eq!(unicode_to_bijoy("ম্ল"), "¤¬");
    assert_eq!(unicode_to_bijoy("অম্ল"), "A¤¬");
}

#[test]
fn test_183() {
    assert_eq!(unicode_to_bijoy("য্য"), "h¨");
    assert_eq!(unicode_to_bijoy("ন্যায্য"), "b¨vh¨");
}

#[test]
fn test_184() {
    assert_eq!(unicode_to_bijoy("র্ক"), "K©");
    assert_eq!(unicode_to_bijoy("তর্ক"), "ZK©");
}

#[test]
fn test_185() {
    assert_eq!(unicode_to_bijoy("র্ক্য"), "K¨©");
    assert_eq!(unicode_to_bijoy("অতর্ক্য"), "AZK¨©");
}

#[test]
fn test_186() {
    assert_eq!(unicode_to_bijoy("র্গ"), "M©");
    assert_eq!(unicode_to_bijoy("বর্গ"), "eM©");
}

#[test]
fn test_187() {
    assert_eq!(unicode_to_bijoy("র্গ্য"), "M¨©");
    assert_eq!(unicode_to_bijoy("বর্গ্য"), "eM¨©");
}

#[test]
fn test_188() {
    assert_eq!(unicode_to_bijoy("র্ঘ্য"), "N¨©");
    assert_eq!(unicode_to_bijoy("দৈর্ঘ্য"), "ˆ`N¨©");
}

#[test]
fn test_189() {
    assert_eq!(unicode_to_bijoy("র্ঙ্গ"), "½©");
    assert_eq!(unicode_to_bijoy("শার্ঙ্গ"), "kv½©");
}

#[test]
fn test_190() {
    assert_eq!(unicode_to_bijoy("র্চ্য"), "P¨©");
    assert_eq!(unicode_to_bijoy("অর্চ্য"), "AP¨©");
}

#[test]
fn test_191() {
    assert_eq!(unicode_to_bijoy("র্জ্য"), "R¨©");
    assert_eq!(unicode_to_bijoy("বর্জ্য"), "eR¨©");
}

#[test]
fn test_192() {
    assert_eq!(unicode_to_bijoy("র্জ্জ"), "¾©");
    assert_eq!(unicode_to_bijoy("ঊর্জ্জ"), "E¾©");
}

#[test]
fn test_193() {
    assert_eq!(unicode_to_bijoy("র্জ্ঞ"), "Á©");
    assert_eq!(unicode_to_bijoy("দুর্জ্ঞেয়"), "`y‡Á©q");
}

#[test]
fn test_194() {
    assert_eq!(unicode_to_bijoy("র্ণ্য"), "Y¨©");
    assert_eq!(unicode_to_bijoy("বৈবর্ণ্য"), "ˆeeY¨©");
}

#[test]
fn test_195() {
    assert_eq!(unicode_to_bijoy("র্ত্য"), "Z¨©");
    assert_eq!(unicode_to_bijoy("মর্ত্য"), "gZ¨©");
}

#[test]
fn test_196() {
    assert_eq!(unicode_to_bijoy("র্থ্য"), "_¨©");
    assert_eq!(unicode_to_bijoy("সামর্থ্য"), "mvg_¨©");
}

#[test]
fn test_197() {
    assert_eq!(unicode_to_bijoy("র্ব্য"), "e¨©");
    assert_eq!(unicode_to_bijoy("নৈর্ব্যক্তিক"), "ˆbe¨©w³K");
}

#[test]
fn test_198() {
    assert_eq!(unicode_to_bijoy("র্ম্য"), "g¨©");
    assert_eq!(unicode_to_bijoy("নৈষ্কর্ম্য"), "ˆb®‹g¨©");
}

#[test]
fn test_199() {
    assert_eq!(unicode_to_bijoy("র্শ্য"), "k¨©");
    assert_eq!(unicode_to_bijoy("অস্পর্শ্য"), "A¯ck¨©");
}

#[test]
fn test_200() {
    assert_eq!(unicode_to_bijoy("র্ষ্য"), "l¨©");
    assert_eq!(unicode_to_bijoy("ঔৎকর্ষ্য"), "JrKl¨©");
}

#[test]
fn test_201() {
    assert_eq!(unicode_to_bijoy("র্হ্য"), "n¨©");
    assert_eq!(unicode_to_bijoy("গর্হ্য"), "Mn¨©");
}

#[test]
fn test_202() {
    assert_eq!(unicode_to_bijoy("র্খ"), "L©");
    assert_eq!(unicode_to_bijoy("মূর্খ"), "g~L©");
}

#[test]
fn test_203() {
    assert_eq!(unicode_to_bijoy("র্গ"), "M©");
    assert_eq!(unicode_to_bijoy("দুর্গ"), "`yM©");
}

#[test]
fn test_204() {
    assert_eq!(unicode_to_bijoy("র্গ্র"), "MÖ©");
    assert_eq!(unicode_to_bijoy("দুর্গ্রহ"), "`yMÖ©n");
    assert_eq!(unicode_to_bijoy("নির্গ্রন্হ"), "wbMÖ©›n");
}

#[test]
fn test_205() {
    assert_eq!(unicode_to_bijoy("র্ঘ"), "N©");
    assert_eq!(unicode_to_bijoy("দীর্ঘ"), "`xN©");
}

#[test]
fn test_206() {
    assert_eq!(unicode_to_bijoy("র্চ"), "P©");
    assert_eq!(unicode_to_bijoy("অর্চনা"), "AP©bv");
}

#[test]
fn test_207() {
    assert_eq!(unicode_to_bijoy("র্ছ"), "Q©");
    assert_eq!(unicode_to_bijoy("মূর্ছনা"), "g~Q©bv");
}

#[test]
fn test_208() {
    assert_eq!(unicode_to_bijoy("র্জ"), "R©");
    assert_eq!(unicode_to_bijoy("অর্জন"), "AR©b");
}

#[test]
fn test_209() {
    assert_eq!(unicode_to_bijoy("র্ঝ"), "S©");
    assert_eq!(unicode_to_bijoy("নির্ঝর"), "wbS©i");
}

#[test]
fn test_210() {
    assert_eq!(unicode_to_bijoy("র্ট"), "U©");
    assert_eq!(unicode_to_bijoy("আর্ট"), "AvU©");
    assert_eq!(unicode_to_bijoy("কোর্ট"), "†KvU©");
    assert_eq!(unicode_to_bijoy("কম্ফর্টার"), "KçU©vi");
    assert_eq!(unicode_to_bijoy("শার্ট"), "kvU©");
    assert_eq!(unicode_to_bijoy("কার্টিজ"), "KvwU©R");
    assert_eq!(unicode_to_bijoy("আর্টিস্ট"), "AvwU©÷");
    assert_eq!(unicode_to_bijoy("পোর্টম্যানটো"), "†cvU©g¨vb‡Uv");
    assert_eq!(unicode_to_bijoy("সার্টিফিকেট"), "mvwU©wd‡KU");
    assert_eq!(unicode_to_bijoy("কনসার্ট"), "KbmvU©");
    assert_eq!(unicode_to_bijoy("কার্টুন"), "KvU©zb");
    assert_eq!(unicode_to_bijoy("কোয়ার্টার"), "†KvqvU©vi");
}

#[test]
fn test_211() {
    assert_eq!(unicode_to_bijoy("র্ড"), "W©");
    assert_eq!(unicode_to_bijoy("অর্ডার"), "AW©vi");
    assert_eq!(unicode_to_bijoy("লর্ড"), "jW©");
    assert_eq!(unicode_to_bijoy("বর্ডার"), "eW©vi");
    assert_eq!(unicode_to_bijoy("কার্ড"), "KvW©");
}

#[test]
fn test_212() {
    assert_eq!(unicode_to_bijoy("র্ণ"), "Y©");
    assert_eq!(unicode_to_bijoy("বর্ণ"), "eY©");
}

#[test]
fn test_213() {
    assert_eq!(unicode_to_bijoy("র্ত"), "Z©");
    assert_eq!(unicode_to_bijoy("ক্ষুধার্ত"), "¶zavZ©");
}

#[test]
fn test_214() {
    assert_eq!(unicode_to_bijoy("র্ত্ম"), "Í©");
    assert_eq!(unicode_to_bijoy("ক্লিন্নবর্ত্মাস্থি"), "wK¬bœeÍ©vw¯’");
}

#[test]
fn test_215() {
    assert_eq!(unicode_to_bijoy("র্ত্র"), "Î©");
    assert_eq!(unicode_to_bijoy("কর্ত্রী"), "KÎ©x");
}

#[test]
fn test_216() {
    assert_eq!(unicode_to_bijoy("র্থ"), "_©");
    assert_eq!(unicode_to_bijoy("অর্থ"), "A_©");
}

#[test]
fn test_217() {
    assert_eq!(unicode_to_bijoy("র্দ"), "`©");
    assert_eq!(unicode_to_bijoy("নির্দয়"), "wb`©q");
}

#[test]
fn test_218() {
    assert_eq!(unicode_to_bijoy("র্দ্ব"), "Ø©");
    assert_eq!(unicode_to_bijoy("নির্দ্বিধা"), "wbwØ©av");
}

#[test]
fn test_219() {
    assert_eq!(unicode_to_bijoy("র্দ্র"), "`ª©");
    assert_eq!(unicode_to_bijoy("আর্দ্র"), "Av`ª©");
}

#[test]
fn test_220() {
    assert_eq!(unicode_to_bijoy("র্ধ"), "a©");
    assert_eq!(unicode_to_bijoy("গোলার্ধ"), "†Mvjva©");
}

#[test]
fn test_221() {
    assert_eq!(unicode_to_bijoy("র্ধ্ব"), "aŸ©");
    assert_eq!(unicode_to_bijoy("ঊর্ধ্ব"), "EaŸ©");
}

#[test]
fn test_222() {
    assert_eq!(unicode_to_bijoy("র্ন"), "b©");
    assert_eq!(unicode_to_bijoy("দুর্নাম"), "`yb©vg");
}

#[test]
fn test_223() {
    assert_eq!(unicode_to_bijoy("র্প"), "c©");
    assert_eq!(unicode_to_bijoy("দর্প"), "`c©");
}

#[test]
fn test_224() {
    assert_eq!(unicode_to_bijoy("র্ফ"), "d©");
    assert_eq!(unicode_to_bijoy("স্কার্ফ"), "¯‹vd©");
}

#[test]
fn test_225() {
    assert_eq!(unicode_to_bijoy("র্ব"), "e©");
    assert_eq!(unicode_to_bijoy("উর্বর"), "De©i");
}

#[test]
fn test_226() {
    assert_eq!(unicode_to_bijoy("র্ভ"), "f©");
    assert_eq!(unicode_to_bijoy("গর্ভ"), "Mf©");
}

#[test]
fn test_227() {
    assert_eq!(unicode_to_bijoy("র্ম"), "g©");
    assert_eq!(unicode_to_bijoy("ধর্ম"), "ag©");
}

#[test]
fn test_228() {
    assert_eq!(unicode_to_bijoy("র্য"), "h©");
    assert_eq!(unicode_to_bijoy("আর্য"), "Avh©");
}

#[test]
fn test_229() {
    assert_eq!(unicode_to_bijoy("র্ল"), "j©");
    assert_eq!(unicode_to_bijoy("দুর্লভ"), "`yj©f");
}

#[test]
fn test_230() {
    assert_eq!(unicode_to_bijoy("র্শ"), "k©");
    assert_eq!(unicode_to_bijoy("স্পর্শ"), "¯ck©");
}

#[test]
fn test_231() {
    assert_eq!(unicode_to_bijoy("র্শ্ব"), "k¦©");
    assert_eq!(unicode_to_bijoy("পার্শ্ব"), "cvk¦©");
}

#[test]
fn test_232() {
    assert_eq!(unicode_to_bijoy("র্ষ"), "l©");
    assert_eq!(unicode_to_bijoy("ঘর্ষণ"), "Nl©Y");
    assert_eq!(unicode_to_bijoy("ধর্ষণ"), "al©Y");
}

#[test]
fn test_233() {
    assert_eq!(unicode_to_bijoy("র্ষ্ট"), "ó©");
    assert_eq!(unicode_to_bijoy("ধার্ষ্টামি"), "avó©vwg");
}

#[test]
fn test_234() {
    assert_eq!(unicode_to_bijoy("র্ষ্ণ"), "ò©");
    assert_eq!(unicode_to_bijoy("পার্ষ্ণিকাস্থি"), "cvwò©Kvw¯’");
}

#[test]
fn test_235() {
    assert_eq!(unicode_to_bijoy("র্ষ্ণ্য"), "ò¨©");
    assert_eq!(unicode_to_bijoy("কার্ষ্ণ্য"), "Kvò¨©");
}

#[test]
fn test_236() {
    assert_eq!(unicode_to_bijoy("র্স"), "m©");
    assert_eq!(unicode_to_bijoy("জার্সি"), "Rvwm©");
    assert_eq!(unicode_to_bijoy("নার্স"), "bvm©");
    assert_eq!(unicode_to_bijoy("পার্সেল"), "cv‡m©j");
    assert_eq!(unicode_to_bijoy("কুর্সি"), "Kzwm©");
}

#[test]
fn test_237() {
    assert_eq!(unicode_to_bijoy("র্হ"), "n©");
    assert_eq!(unicode_to_bijoy("গার্হস্থ্য"), "Mvn©¯’¨");
}

#[test]
fn test_238() {
    assert_eq!(unicode_to_bijoy("র্হ্য"), "n¨©");
    assert_eq!(unicode_to_bijoy("গর্হ্য"), "Mn¨©");
}

#[test]
fn test_239() {
    assert_eq!(unicode_to_bijoy("র্ঢ্য"), "X¨©");
    assert_eq!(unicode_to_bijoy("দার্ঢ্য"), "`vX¨©");
}

#[test]
fn test_240() {
    assert_eq!(unicode_to_bijoy("ল্ক"), "é");
    assert_eq!(unicode_to_bijoy("শুল্ক"), "ïé");
}

#[test]
fn test_241() {
    assert_eq!(unicode_to_bijoy("ল্ক্য"), "é¨");
    assert_eq!(unicode_to_bijoy("যাজ্ঞবল্ক্য"), "hvÁeé¨");
}

#[test]
fn test_242() {
    assert_eq!(unicode_to_bijoy("ল্গ"), "ê");
    assert_eq!(unicode_to_bijoy("বল্গা"), "eêv");
}

#[test]
fn test_243() {
    assert_eq!(unicode_to_bijoy("ল্ট"), "ë");
    assert_eq!(unicode_to_bijoy("উল্টো"), "D‡ëv");
}

#[test]
fn test_244() {
    assert_eq!(unicode_to_bijoy("ল্ড"), "ì");
    assert_eq!(unicode_to_bijoy("ফিল্ডিং"), "wdwìs");
}

#[test]
fn test_245() {
    assert_eq!(unicode_to_bijoy("ল্প"), "í");
    assert_eq!(unicode_to_bijoy("বিকল্প"), "weKí");
}

#[test]
fn test_246() {
    assert_eq!(unicode_to_bijoy("ল্ফ"), "î");
    assert_eq!(unicode_to_bijoy("গুল্ফ"), "¸î");
}

#[test]
fn test_247() {
    assert_eq!(unicode_to_bijoy("ল্ব"), "j¡");
    assert_eq!(unicode_to_bijoy("বিল্ব"), "wej¡");
    assert_eq!(unicode_to_bijoy("বাল্ব"), "evj¡");
}

#[test]
fn test_248() {
    assert_eq!(unicode_to_bijoy("ল্ভ"), "j¢");
    assert_eq!(unicode_to_bijoy("প্রগল্ভ"), "cÖMj¢");
}

#[test]
fn test_249() {
    assert_eq!(unicode_to_bijoy("ল্ম"), "j¥");
    assert_eq!(unicode_to_bijoy("গুল্ম"), "¸j¥");
}

#[test]
fn test_250() {
    assert_eq!(unicode_to_bijoy("ল্য"), "j¨");
    assert_eq!(unicode_to_bijoy("তারল্য"), "Zvij¨");
}

#[test]
fn test_251() {
    assert_eq!(unicode_to_bijoy("ল্ল"), "j−");
    assert_eq!(unicode_to_bijoy("উল্লাস"), "Dj−vm");
}

#[test]
fn test_252() {
    assert_eq!(unicode_to_bijoy("শ্চ"), "ð");
    assert_eq!(unicode_to_bijoy("পুনশ্চ"), "cybð");
}

#[test]
fn test_253() {
    assert_eq!(unicode_to_bijoy("শ্ছ"), "ñ");
    assert_eq!(unicode_to_bijoy("শিরশ্ছেদ"), "wki‡ñ`");
}

#[test]
fn test_254() {
    assert_eq!(unicode_to_bijoy("শ্ন"), "kœ");
    assert_eq!(unicode_to_bijoy("প্রশ্ন"), "cÖkœ");
}

#[test]
fn test_255() {
    assert_eq!(unicode_to_bijoy("শ্ব"), "k¦");
    assert_eq!(unicode_to_bijoy("বিশ্ব"), "wek¦");
    assert_eq!(unicode_to_bijoy("অশ্ব"), "Ak¦");
}

#[test]
fn test_256() {
    assert_eq!(unicode_to_bijoy("শ্ম"), "k¥");
    assert_eq!(unicode_to_bijoy("জীবাশ্ম"), "Rxevk¥");
}

#[test]
fn test_257() {
    assert_eq!(unicode_to_bijoy("শ্য"), "k¨");
    assert_eq!(unicode_to_bijoy("অবশ্য"), "Aek¨");
}

#[test]
fn test_258() {
    assert_eq!(unicode_to_bijoy("শ্র"), "kª");
    assert_eq!(unicode_to_bijoy("মিশ্র"), "wgkª");
}

#[test]
fn test_259() {
    assert_eq!(unicode_to_bijoy("শ্ল"), "k−");
    assert_eq!(unicode_to_bijoy("অশ্লীল"), "Ak−xj");
    assert_eq!(unicode_to_bijoy("শ্লোক"), "†k−vK");
}

#[test]
fn test_260() {
    assert_eq!(unicode_to_bijoy("ষ্ক"), "®‹");
    assert_eq!(unicode_to_bijoy("শুষ্ক"), "ï®‹");
}

#[test]
fn test_261() {
    assert_eq!(unicode_to_bijoy("ষ্ক্ব"), "®‹¡");
    assert_eq!(unicode_to_bijoy("নিষ্ক্বাথ"), "wb®‹¡v_");
}

#[test]
fn test_262() {
    assert_eq!(unicode_to_bijoy("ষ্ক্র"), "®Œ");
    assert_eq!(unicode_to_bijoy("নিষ্ক্রিয়"), "wbw®Œq");
}

#[test]
fn test_263() {
    assert_eq!(unicode_to_bijoy("ষ্ট"), "ó");
    assert_eq!(unicode_to_bijoy("কষ্ট"), "Kó");
}

#[test]
fn test_264() {
    assert_eq!(unicode_to_bijoy("ষ্ট্য"), "ó¨");
    assert_eq!(unicode_to_bijoy("বৈশিষ্ট্য"), "ˆewkó¨");
}

#[test]
fn test_265() {
    assert_eq!(unicode_to_bijoy("ষ্ট্র"), "óª");
    assert_eq!(unicode_to_bijoy("রাষ্ট্র"), "ivóª");
}

#[test]
fn test_266() {
    assert_eq!(unicode_to_bijoy("ষ্ঠ"), "ô");
    assert_eq!(unicode_to_bijoy("শ্রেষ্ঠ"), "†kªô");
}

#[test]
fn test_267() {
    assert_eq!(unicode_to_bijoy("ষ্ঠ্য"), "ô¨");
    assert_eq!(unicode_to_bijoy("নিষ্ঠ্যূত"), "wbô¨~Z");
}

#[test]
fn test_268() {
    assert_eq!(unicode_to_bijoy("ষ্ণ"), "ò");
    assert_eq!(unicode_to_bijoy("কৃষ্ণ"), "K…ò");
}

#[test]
fn test_269() {
    assert_eq!(unicode_to_bijoy("ষ্ণ্ব"), "ò¡");
    assert_eq!(unicode_to_bijoy("বিষ্ণ্বিন্দ্র"), "wewò¡›`ª");
}

#[test]
fn test_270() {
    assert_eq!(unicode_to_bijoy("ষ্প"), "®c");
    assert_eq!(unicode_to_bijoy("নিষ্পাপ"), "wb®cvc");
}

#[test]
fn test_271() {
    assert_eq!(unicode_to_bijoy("ষ্প্র"), "®cÖ");
    assert_eq!(unicode_to_bijoy("নিষ্প্রয়োজন"), "wb®cÖ‡qvRb");
}

#[test]
fn test_272() {
    assert_eq!(unicode_to_bijoy("ষ্ফ"), "õ");
    assert_eq!(unicode_to_bijoy("নিষ্ফল"), "wbõj");
}

#[test]
fn test_273() {
    assert_eq!(unicode_to_bijoy("ষ্ব"), "®^");
    assert_eq!(unicode_to_bijoy("মাতৃষ্বসা"), "gvZ…®^mv");
}

#[test]
fn test_274() {
    assert_eq!(unicode_to_bijoy("ষ্ম"), "®§");
    assert_eq!(unicode_to_bijoy("উষ্ম"), "D®§");
}

#[test]
fn test_275() {
    assert_eq!(unicode_to_bijoy("ষ্য"), "l¨");
    assert_eq!(unicode_to_bijoy("শিষ্য"), "wkl¨");
}

#[test]
fn test_276() {
    assert_eq!(unicode_to_bijoy("স্ক"), "¯‹");
    assert_eq!(unicode_to_bijoy("মনোস্কামনা"), "g‡bv¯‹vgbv");
}

#[test]
fn test_277() {
    assert_eq!(unicode_to_bijoy("স্ক্র"), "¯Œ");
    assert_eq!(unicode_to_bijoy("ইস্ক্রু"), "B¯Œy");
}

#[test]
fn test_278() {
    assert_eq!(unicode_to_bijoy("স্খ"), "ö");
    assert_eq!(unicode_to_bijoy("স্খলন"), "öjb");
}

#[test]
fn test_279() {
    assert_eq!(unicode_to_bijoy("স্ট"), "÷");
    assert_eq!(unicode_to_bijoy("স্টেশন"), "†÷kb");
}

#[test]
fn test_280() {
    assert_eq!(unicode_to_bijoy("স্ট্র"), "÷ª");
    assert_eq!(unicode_to_bijoy("স্ট্রাইক"), "÷ªvBK");
}

#[test]
fn test_281() {
    assert_eq!(unicode_to_bijoy("স্ত"), "¯—");
    assert_eq!(unicode_to_bijoy("ব্যস্ত"), "e¨¯—");
    assert_eq!(unicode_to_bijoy("ন্যস্ত"), "b¨¯—");
}

#[test]
fn test_282() {
    assert_eq!(unicode_to_bijoy("স্ত্ব"), "¯—¡");
    assert_eq!(unicode_to_bijoy("বহিস্ত্বক"), "ewn¯—¡K");
}

#[test]
fn test_283() {
    assert_eq!(unicode_to_bijoy("স্ত্য"), "¯—¨");
    assert_eq!(unicode_to_bijoy("অস্ত্যর্থ"), "A¯—¨_©");
}

#[test]
fn test_284() {
    assert_eq!(unicode_to_bijoy("স্ত্র"), "¯¿");
    assert_eq!(unicode_to_bijoy("স্ত্রী"), "¯¿x");
}

#[test]
fn test_285() {
    assert_eq!(unicode_to_bijoy("স্থ"), "¯’");
    assert_eq!(unicode_to_bijoy("দুঃস্থ"), "`yt¯’");
}

#[test]
fn test_286() {
    assert_eq!(unicode_to_bijoy("স্থ্য"), "¯’¨");
    assert_eq!(unicode_to_bijoy("স্বাস্থ্য"), "¯^v¯’¨");
}

#[test]
fn test_287() {
    assert_eq!(unicode_to_bijoy("স্ন"), "ø");
    assert_eq!(unicode_to_bijoy("স্নান"), "øvb");
}

#[test]
fn test_288() {
    assert_eq!(unicode_to_bijoy("স্ন্য"), "ø¨");
}

#[test]
fn test_289() {
    assert_eq!(unicode_to_bijoy("স্প"), "¯c");
    assert_eq!(unicode_to_bijoy("আস্পর্ধা"), "Av¯ca©v");
}

#[test]
fn test_290() {
    assert_eq!(unicode_to_bijoy("স্প্র"), "¯cÖ");
    assert_eq!(unicode_to_bijoy("স্প্রিং"), "w¯cÖs");
}

#[test]
fn test_291() {
    assert_eq!(unicode_to_bijoy("স্প্ল"), "¯c−");
    assert_eq!(unicode_to_bijoy("স্প্লিন"), "w¯c−b");
}

#[test]
fn test_292() {
    assert_eq!(unicode_to_bijoy("স্ফ"), "ù");
    assert_eq!(unicode_to_bijoy("আস্ফালন"), "Avùvjb");
}

#[test]
fn test_293() {
    assert_eq!(unicode_to_bijoy("স্ব"), "¯^");
    assert_eq!(unicode_to_bijoy("স্বর"), "¯^i");
}

#[test]
fn test_294() {
    assert_eq!(unicode_to_bijoy("স্ম"), "¯§");
    assert_eq!(unicode_to_bijoy("স্মরণ"), "¯§iY");
}

#[test]
fn test_295() {
    assert_eq!(unicode_to_bijoy("স্য"), "m¨");
    assert_eq!(unicode_to_bijoy("শস্য"), "km¨");
}

#[test]
fn test_296() {
    assert_eq!(unicode_to_bijoy("স্র"), "mª");
    assert_eq!(unicode_to_bijoy("অজস্র"), "ARmª");
}

#[test]
fn test_297() {
    assert_eq!(unicode_to_bijoy("স্ল"), "¯¬");
    assert_eq!(unicode_to_bijoy("স্লোগান"), "†¯¬vMvb");
}

#[test]
fn test_298() {
    assert_eq!(unicode_to_bijoy("হ্ণ"), "nœ");
    assert_eq!(unicode_to_bijoy("অপরাহ্ণ"), "Acivnœ");
}

#[test]
fn test_299() {
    assert_eq!(unicode_to_bijoy("হ্ন"), "ý");
    assert_eq!(unicode_to_bijoy("চিহ্ন"), "wPý");
}

#[test]
fn test_300() {
    assert_eq!(unicode_to_bijoy("হ্ব"), "nŸ");
    assert_eq!(unicode_to_bijoy("আহ্বান"), "AvnŸvb");
}

#[test]
fn test_301() {
    assert_eq!(unicode_to_bijoy("হ্ম"), "þ");
    assert_eq!(unicode_to_bijoy("ব্রাহ্মণ"), "eªvþY");
}

#[test]
fn test_302() {
    assert_eq!(unicode_to_bijoy("হ্য"), "n¨");
    assert_eq!(unicode_to_bijoy("বাহ্য"), "evn¨");
    assert_eq!(unicode_to_bijoy("সহ্য"), "mn¨");
}

#[test]
fn test_303() {
    assert_eq!(unicode_to_bijoy("হ্র"), "nª");
    assert_eq!(unicode_to_bijoy("হ্রদ"), "nª`");
}

#[test]
fn test_304() {
    assert_eq!(unicode_to_bijoy("হ্ল"), "n¬");
    assert_eq!(unicode_to_bijoy("আহ্লাদ"), "Avn¬v`");
    assert_eq!(unicode_to_bijoy("প্রহ্লাদ"), "cÖn¬v`");
}

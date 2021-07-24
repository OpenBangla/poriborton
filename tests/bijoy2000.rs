use poriborton::bijoy2000::Bijoy2000;
use pretty_assertions::assert_eq;

#[test]
fn test_1() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ক"), "°");
    assert_eq!(converter.convert("আক্কেল"), "Av‡°j");
    assert_eq!(converter.convert("টেক্কা"), "†U°v");
    assert_eq!(converter.convert("ধাক্কা"), "av°v");
}

#[test]
fn test_2() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ট"), "±");
    assert_eq!(converter.convert("ডক্টর"), "W±i");
    assert_eq!(converter.convert("অক্টোপাস"), "A‡±vcvm");
}

#[test]
fn test_3() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ট্র"), "±ª");
    assert_eq!(converter.convert("অক্ট্রয়"), "A±ªq");
}

#[test]
fn test_4() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ত"), "³");
    assert_eq!(converter.convert("রক্ত"), "i³");
    assert_eq!(converter.convert("শক্ত"), "k³");
}

#[test]
fn test_5() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ত্র"), "³«");
    assert_eq!(converter.convert("বক্ত্র"), "e³«");
}

#[test]
fn test_6() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ন"), "Kè");
    assert_eq!(converter.convert("বাচক্নবী"), "evPKèex");
}

#[test]
fn test_7() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ব"), "K¡");
    assert_eq!(converter.convert("পক্ব"), "cK¡");
    assert_eq!(converter.convert("ক্বণ"), "K¡Y");
}

#[test]
fn test_8() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ম"), "´");
    assert_eq!(converter.convert("রুক্মিণী"), "i“w´Yx");
}

#[test]
fn test_9() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্য"), "K¨");
    assert_eq!(converter.convert("বাক্য"), "evK¨");
}

#[test]
fn test_10() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্র"), "µ");
    assert_eq!(converter.convert("চক্র"), "Pµ");
    assert_eq!(converter.convert("বক্র"), "eµ");
}

#[test]
fn test_11() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ল"), "K¬");
    assert_eq!(converter.convert("ক্লান্তি"), "K¬vwš—");
}

#[test]
fn test_12() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ"), "¶");
    assert_eq!(converter.convert("পক্ষ"), "c¶");
    assert_eq!(converter.convert("ভক্ষক"), "f¶K");
    assert_eq!(converter.convert("অপেক্ষা"), "A‡c¶v");
}

#[test]
fn test_13() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ্ণ"), "¶è");
    assert_eq!(converter.convert("তীক্ষ্ণ"), "Zx¶è");
}

#[test]
fn test_14() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ্ব"), "¶¡");
    assert_eq!(converter.convert("ইক্ষ্বাকু"), "B¶¡vKz");
}

#[test]
fn test_15() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ্ম"), "²");
    assert_eq!(converter.convert("লক্ষ্মী"), "j²x");
}

#[test]
fn test_16() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ্ম্য"), "²¨");
    assert_eq!(converter.convert("সৌক্ষ্ম্য"), "†mŠ²¨");
}

#[test]
fn test_17() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্ষ্য"), "¶¨");
    assert_eq!(converter.convert("লক্ষ্য"), "j¶¨");
}

#[test]
fn test_18() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ক্স"), "·");
    assert_eq!(converter.convert("বাক্স"), "ev·");
}

#[test]
fn test_19() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("খ্য"), "L¨");
    assert_eq!(converter.convert("সখ্য"), "mL¨");
    assert_eq!(converter.convert("সাংখ্য"), "mvsL¨");
}

#[test]
fn test_20() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("খ্র"), "Lª");
    assert_eq!(converter.convert("খ্রিস্টান"), "wLª÷vb");
}

#[test]
fn test_21() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্‌ণ"), "M&Y");
    assert_eq!(converter.convert("রুগ্‌ণ"), "i“M&Y");
}

#[test]
fn test_22() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ধ"), "»");
    assert_eq!(converter.convert("মুগ্ধ"), "gy»");
}

#[test]
fn test_23() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ধ্য"), "»¨");
    assert_eq!(converter.convert("বৈদগ্ধ্য"), "ˆe`»¨");
}

#[test]
fn test_24() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ধ্র"), "»ª");
    assert_eq!(converter.convert("দোগ্ধ্রী"), "†`v»ªx");
}

#[test]
fn test_25() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ন"), "Mœ");
    assert_eq!(converter.convert("ভগ্ন"), "fMœ");
    assert_eq!(converter.convert("অগ্নি"), "AwMœ");
}

#[test]
fn test_26() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ন্য"), "Mœ¨");
    assert_eq!(converter.convert("অগ্ন্যাস্ত্র"), "AMœ¨v¯¿");
    assert_eq!(converter.convert("অগ্ন্যুৎপাত"), "AMœ¨yrcvZ");
    assert_eq!(converter.convert("অগ্ন্যাশয়"), "AMœ¨vkq");
}

#[test]
fn test_27() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ব"), "M¦");
    assert_eq!(converter.convert("দিগ্বিজয়ী"), "w`wM¦Rqx");
}

#[test]
fn test_28() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ম"), "M¥");
    assert_eq!(converter.convert("যুগ্ম"), "hyM¥");
}

#[test]
fn test_29() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্য"), "M¨");
    assert_eq!(converter.convert("ভাগ্য"), "fvM¨");
}

#[test]
fn test_30() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্র"), "MÖ");
    assert_eq!(converter.convert("গ্রাম"), "MÖvg");
}

#[test]
fn test_31() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্র্য"), "MÖ¨");
    assert_eq!(converter.convert("ঐকাগ্র্য"), "HKvMÖ¨");
    assert_eq!(converter.convert("সামগ্র্য"), "mvgMÖ¨");
    assert_eq!(converter.convert("গ্র্যাজুয়েট"), "MÖ¨vRz‡qU");
}

#[test]
fn test_32() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("গ্ল"), "M−");
    assert_eq!(converter.convert("গ্লানি"), "M−vwb");
}

#[test]
fn test_33() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঘ্ন"), "Nœ");
    assert_eq!(converter.convert("কৃতঘ্ন"), "K…ZNœ");
    assert_eq!(converter.convert("শত্রুঘ্ন"), "kÎ“Nœ");
}

#[test]
fn test_34() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঘ্য"), "N¨");
    assert_eq!(converter.convert("অশ্লাঘ্য"), "Ak−vN¨");
}

#[test]
fn test_35() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঘ্র"), "Nª");
    assert_eq!(converter.convert("ঘ্রাণ"), "NªvY");
}

#[test]
fn test_36() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ক"), "¼");
    assert_eq!(converter.convert("অঙ্ক"), "A¼");
    assert_eq!(converter.convert("টঙ্কা"), "U¼v");
    assert_eq!(converter.convert("শশাঙ্ক"), "kkv¼");
}

#[test]
fn test_37() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্‌ক্ত"), "O&³");
    assert_eq!(converter.convert("পঙ্‌ক্তি"), "cO&w³");
}

#[test]
fn test_38() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ক্য"), "¼¨");
    assert_eq!(converter.convert("অঙ্ক্য"), "A¼¨");
}

#[test]
fn test_39() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ক্ষ"), "•¶");
    assert_eq!(converter.convert("আকাঙ্ক্ষা"), "AvKv•¶v");
}

#[test]
fn test_40() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্খ"), "•L");
    assert_eq!(converter.convert("শঙ্খ"), "k•L");
}

#[test]
fn test_41() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্খ্য"), "•L¨");
    assert_eq!(converter.convert("সাঙ্খ্যমান"), "mv•L¨gvb");
}

#[test]
fn test_42() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্গ"), "½");
    assert_eq!(converter.convert("অঙ্গ"), "A½");
    assert_eq!(converter.convert("সঙ্গী"), "m½x");
}

#[test]
fn test_43() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্গ্য"), "½¨");
    assert_eq!(converter.convert("ব্যঙ্গ্যার্থ"), "e¨½¨v_©");
    assert_eq!(converter.convert("ব্যঙ্গ্যোক্তি"), "e¨‡½¨vw³");
}

#[test]
fn test_44() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ঘ"), "•N");
    assert_eq!(converter.convert("সঙ্ঘ"), "m•N");
}

#[test]
fn test_45() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ঘ্য"), "•N¨");
    assert_eq!(converter.convert("দুর্লঙ্ঘ্য"), "`yj©•N¨");
}

#[test]
fn test_46() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ঘ্র"), "•Nª");
    assert_eq!(converter.convert("অঙ্ঘ্রি"), "Aw•Nª");
}

#[test]
fn test_47() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঙ্ম"), "•g");
    assert_eq!(converter.convert("বাঙ্ময়"), "ev•gq");
}

#[test]
fn test_48() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্চ"), "”P");
    assert_eq!(converter.convert("বাচ্চা"), "ev”Pv");
}

#[test]
fn test_49() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্ছ"), "”Q");
    assert_eq!(converter.convert("ইচ্ছা"), "B”Qv");
}

#[test]
fn test_50() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্ছ্ব"), "”Q¡");
    assert_eq!(converter.convert("জলোচ্ছ্বাস"), "R‡jv”Q¡vm");
}

#[test]
fn test_51() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্ছ্র"), "”Qª");
    assert_eq!(converter.convert("উচ্ছ্রায়"), "D”Qªvq");
}

#[test]
fn test_52() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্ঞ"), "”T");
    assert_eq!(converter.convert("যাচ্ঞা"), "hv”Tv");
}

#[test]
fn test_53() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্ব"), "”¡");
    assert_eq!(converter.convert("চ্বী"), "”¡x");
}

#[test]
fn test_54() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("চ্য"), "P¨");
    assert_eq!(converter.convert("প্রাচ্য"), "cÖvP¨");
}

#[test]
fn test_55() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্জ"), "¾");
    assert_eq!(converter.convert("বিপজ্জনক"), "wec¾bK");
}

#[test]
fn test_56() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্জ্ব"), "¾¡");
    assert_eq!(converter.convert("উজ্জ্বল"), "D¾¡j");
}

#[test]
fn test_57() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্ঝ"), "À");
    assert_eq!(converter.convert("কুজ্ঝটিকা"), "KzÀwUKv");
}

#[test]
fn test_58() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্ঞ"), "Á");
    assert_eq!(converter.convert("জ্ঞান"), "Ávb");
    assert_eq!(converter.convert("সজ্ঞাত"), "mÁvZ");
}

#[test]
fn test_59() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্ব"), "R¡");
    assert_eq!(converter.convert("জ্বর"), "R¡i");
}

#[test]
fn test_60() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্য"), "R¨");
    assert_eq!(converter.convert("রাজ্য"), "ivR¨");
}

#[test]
fn test_61() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("জ্র"), "Rª");
    assert_eq!(converter.convert("বজ্র"), "eRª");
}

#[test]
fn test_62() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঞ্চ"), "Â");
    assert_eq!(converter.convert("অঞ্চল"), "AÂj");
    assert_eq!(converter.convert("সঞ্চয়"), "mÂq");
}

#[test]
fn test_63() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঞ্ছ"), "Ã");
    assert_eq!(converter.convert("লাঞ্ছনা"), "jvÃbv");
    assert_eq!(converter.convert("বাঞ্ছা"), "evÃv");
}

#[test]
fn test_64() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঞ্জ"), "Ä");
    assert_eq!(converter.convert("কুঞ্জ"), "KzÄ");
    assert_eq!(converter.convert("গুঞ্জন"), "¸Äb");
}

#[test]
fn test_65() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঞ্ঝ"), "Å");
    assert_eq!(converter.convert("ঝঞ্ঝা"), "SÅv");
}

#[test]
fn test_66() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ট্ট"), "Æ");
    assert_eq!(converter.convert("চট্টগ্রাম"), "PÆMÖvg");
    assert_eq!(converter.convert("ছোট্ট"), "†QvÆ");
}

#[test]
fn test_67() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ট্ব"), "U¡");
    assert_eq!(converter.convert("খট্বা"), "LU¡v");
}

#[test]
fn test_68() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ট্ম"), "U¥");
    assert_eq!(converter.convert("কুট্মল"), "KzU¥j");
}

#[test]
fn test_69() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ট্য"), "U¨");
    assert_eq!(converter.convert("নাট্য"), "bvU¨");
}

#[test]
fn test_70() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ট্র"), "Uª");
    assert_eq!(converter.convert("ট্রেন"), "†Uªb");
}

#[test]
fn test_71() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড্ড"), "Ç");
    assert_eq!(converter.convert("আড্ডা"), "AvÇv");
    assert_eq!(converter.convert("উড্ডয়ন"), "DÇqb");
}

#[test]
fn test_72() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড্ব"), "W¡");
    assert_eq!(converter.convert("অন্ড্বান"), "AÛ¡vb");
}

#[test]
fn test_73() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড্ম"), "W¥");
    assert_eq!(converter.convert("কুড্মল"), "KzW¥j");
}

#[test]
fn test_74() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড্য"), "W¨");
    assert_eq!(converter.convert("জাড্য"), "RvW¨");
}

#[test]
fn test_75() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড্র"), "Wª");
    assert_eq!(converter.convert("ড্রাইভার"), "WªvBfvi");
    assert_eq!(converter.convert("ড্রাম"), "Wªvg");
}

#[test]
fn test_76() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ড়্গ"), "ÿ");
    assert_eq!(converter.convert("খড়্গ"), "Lÿ");
}

#[test]
fn test_77() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঢ্য"), "X¨");
    assert_eq!(converter.convert("ধনাঢ্য"), "abvX¨");
}

#[test]
fn test_78() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ঢ্র"), "Xª");
    assert_eq!(converter.convert("মেঢ্র"), "†gXª");
}

#[test]
fn test_79() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ট"), "È");
    assert_eq!(converter.convert("ঘণ্টা"), "NÈv");
    assert_eq!(converter.convert("নির্ঘণ্ট"), "wbN©È");
}

#[test]
fn test_80() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ঠ"), "É");
    assert_eq!(converter.convert("কণ্ঠ"), "KÉ");
}

#[test]
fn test_81() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ঠ্য"), "É¨");
    assert_eq!(converter.convert("কণ্ঠ্য"), "KÉ¨");
}

#[test]
fn test_82() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ড"), "Ê");
    assert_eq!(converter.convert("গণ্ডগোল"), "MÊ‡Mvj");
    assert_eq!(converter.convert("কুণ্ড"), "KzÊ");
}

#[test]
fn test_83() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ড্য"), "Ê¨");
    assert_eq!(converter.convert("পাণ্ড্য"), "cvÊ¨");
}

#[test]
fn test_84() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ড্র"), "Êª");
    assert_eq!(converter.convert("পুণ্ড্র"), "cyÊª");
}

#[test]
fn test_85() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ঢ"), "YX");
    assert_eq!(converter.convert("ষণ্ঢ"), "lYX");
}

#[test]
fn test_86() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ণ"), "Yè");
    assert_eq!(converter.convert("বিষণ্ণ"), "welYè");
}

#[test]
fn test_87() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ব"), "Y¡");
    assert_eq!(converter.convert("স্থাণ্বীশ্বর"), "¯’vY¡xk¦i");
}

#[test]
fn test_88() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্ম"), "Y¥");
    assert_eq!(converter.convert("চিণ্ময়"), "wPY¥q");
    assert_eq!(converter.convert("মৃণ্ময়ী"), "g„Y¥qx");
}

#[test]
fn test_89() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ণ্য"), "Y¨");
    assert_eq!(converter.convert("পূণ্য"), "c~Y¨");
}

#[test]
fn test_90() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ৎক"), "rK");
    assert_eq!(converter.convert("উৎকর্ষ"), "DrKl©");
}

#[test]
fn test_91() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ৎখ"), "rL");
    assert_eq!(converter.convert("উৎখাত"), "DrLvZ");
}

#[test]
fn test_92() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ত"), "Ë");
    assert_eq!(converter.convert("উত্তর"), "DËi");
}

#[test]
fn test_93() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ত্ব"), "Ë¡");
    assert_eq!(converter.convert("সত্ত্ব"), "mË¡");
}

#[test]
fn test_94() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ত্য"), "Ë¨");
    assert_eq!(converter.convert("উত্ত্যক্ত"), "DË¨³");
}

#[test]
fn test_95() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্থ"), "Ì");
    assert_eq!(converter.convert("অশ্বত্থ"), "Ak¦Ì");
}

#[test]
fn test_96() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ন"), "Zœ");
    assert_eq!(converter.convert("যত্ন"), "hZœ");
    assert_eq!(converter.convert("রত্নাকর"), "iZœvKi");
}

#[test]
fn test_97() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ৎপ"), "rc");
    assert_eq!(converter.convert("উৎপল"), "Drcj");
}

#[test]
fn test_98() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ব"), "Z¡");
    assert_eq!(converter.convert("রাজত্ব"), "ivRZ¡");
}

#[test]
fn test_99() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ম"), "Í");
    assert_eq!(converter.convert("আত্মা"), "AvÍv");
}

#[test]
fn test_100() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্ম্য"), "Í¨");
    assert_eq!(converter.convert("দৌরাত্ম্য"), "†`ŠivÍ¨");
}

#[test]
fn test_101() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্য"), "Z¨");
    assert_eq!(converter.convert("সত্য"), "mZ¨");
}

#[test]
fn test_102() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্র"), "Î");
    assert_eq!(converter.convert("ত্রিশ"), "wÎk");
    assert_eq!(converter.convert("ত্রাণ"), "ÎvY");
}

#[test]
fn test_103() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ত্র্য"), "Î¨");
    assert_eq!(converter.convert("বৈচিত্র্য"), "ˆewPÎ¨");
}

#[test]
fn test_104() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ৎল"), "rj");
    assert_eq!(converter.convert("কাৎলা"), "Kvrjv");
}

#[test]
fn test_105() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ৎস"), "rm");
    assert_eq!(converter.convert("বৎসর"), "ermi");
    assert_eq!(converter.convert("উৎসব"), "Drme");
}

#[test]
fn test_106() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("থ্ব"), "_¡");
    assert_eq!(converter.convert("পৃথ্বী"), "c„_¡x");
}

#[test]
fn test_107() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("থ্য"), "_¨");
    assert_eq!(converter.convert("পথ্য"), "c_¨");
}

#[test]
fn test_108() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("থ্র"), "_ª");
    assert_eq!(converter.convert("থ্রি"), "w_ª");
}

#[test]
fn test_109() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্গ"), "˜M");
    assert_eq!(converter.convert("উদ্গম"), "D˜Mg");
}

#[test]
fn test_110() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ঘ"), "™N");
    assert_eq!(converter.convert("উদ্ঘাটন"), "D™NvUb");
}

#[test]
fn test_111() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্দ"), "Ï");
    assert_eq!(converter.convert("উদ্দেশ্য"), "D‡Ïk¨");
}

#[test]
fn test_112() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্দ্ব"), "Ï¡");
    assert_eq!(converter.convert("তদ্দ্বারা"), "ZÏ¡viv");
}

#[test]
fn test_113() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ধ"), "×");
    assert_eq!(converter.convert("রুদ্ধ"), "i“×");
    assert_eq!(converter.convert("বদ্ধ"), "e×");
}

#[test]
fn test_114() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ব"), "Ø");
    assert_eq!(converter.convert("বিদ্বান"), "weØvb");
}

#[test]
fn test_115() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ভ"), "™¢");
    assert_eq!(converter.convert("অদ্ভুত"), "A™¢zZ");
}

#[test]
fn test_116() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ভ্র"), "™£");
    assert_eq!(converter.convert("উদ্ভ্রান্ত"), "D™£vš—");
}

#[test]
fn test_117() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্ম"), "Ù");
    assert_eq!(converter.convert("ছদ্ম"), "QÙ");
    assert_eq!(converter.convert("পদ্ম"), "cÙ");
}

#[test]
fn test_118() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্য"), "`¨");
    assert_eq!(converter.convert("বাদ্য"), "ev`¨");
    assert_eq!(converter.convert("পদ্য"), "c`¨");
}

#[test]
fn test_119() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্র"), "`ª");
    assert_eq!(converter.convert("রুদ্র"), "i“`ª");
    assert_eq!(converter.convert("নিদ্রিত"), "wbw`ªZ");
}

#[test]
fn test_120() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("দ্র্য"), "`ª¨");
    assert_eq!(converter.convert("দারিদ্র্য"), "`vwi`ª¨");
}

#[test]
fn test_121() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ধ্ন"), "aœ");
    assert_eq!(converter.convert("অর্থগৃধ্নু"), "A_©M„aœy");
}

#[test]
fn test_122() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ধ্ব"), "aŸ");
    assert_eq!(converter.convert("ধ্বনি"), "aŸwb");
}

#[test]
fn test_123() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ধ্ম"), "a¥");
    assert_eq!(converter.convert("উদরাধ্মান"), "D`iva¥vb");
}

#[test]
fn test_124() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ধ্য"), "a¨");
    assert_eq!(converter.convert("আরাধ্য"), "Aviva¨");
}

#[test]
fn test_125() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ধ্র"), "aª");
    assert_eq!(converter.convert("ধ্রুব"), "aª“e");
}

#[test]
fn test_126() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ট"), "›U");
    assert_eq!(converter.convert("প্যান্ট"), "c¨v›U");
}

#[test]
fn test_127() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ট্র"), "›Uª");
    assert_eq!(converter.convert("কন্ট্রোল"), "K‡›Uªvj");
}

#[test]
fn test_128() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ঠ"), "Ú");
    assert_eq!(converter.convert("লন্ঠন"), "jÚb");
}

#[test]
fn test_129() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ড"), "Û");
    assert_eq!(converter.convert("গন্ডার"), "MÛvi");
    assert_eq!(converter.convert("পাউন্ড"), "cvDÛ");
}

#[test]
fn test_130() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ড্র"), "Ûª");
    assert_eq!(converter.convert("হান্ড্রেড"), "nv‡ÛªW");
}

#[test]
fn test_131() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ত"), "š—");
    assert_eq!(converter.convert("জীবন্ত"), "Rxeš—");
    assert_eq!(converter.convert("গন্তব্য"), "Mš—e¨");
}

#[test]
fn test_132() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ত্ব"), "š—¡");
    assert_eq!(converter.convert("সান্ত্বনা"), "mvš—¡bv");
}

#[test]
fn test_133() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ত্য"), "š—¨");
    assert_eq!(converter.convert("অন্ত্য"), "Aš—¨");
}

#[test]
fn test_134() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ত্র"), "š¿");
    assert_eq!(converter.convert("মন্ত্র"), "gš¿");
    assert_eq!(converter.convert("যন্ত্র"), "hš¿");
}

#[test]
fn test_135() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ত্র্য"), "š¿¨");
    assert_eq!(converter.convert("স্বাতন্ত্র্য"), "¯^vZš¿¨");
}

#[test]
fn test_136() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্থ"), "š’");
    assert_eq!(converter.convert("গ্রন্থ"), "MÖš’");
    assert_eq!(converter.convert("পান্থ"), "cvš’");
}

#[test]
fn test_137() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্থ্র"), "š’ª");
    assert_eq!(converter.convert("অ্যান্থ্রাক্স"), "A¨vš’ªv·");
}

#[test]
fn test_138() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্দ"), "›`");
    assert_eq!(converter.convert("ছন্দ"), "Q›`");
}

#[test]
fn test_139() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্দ্য"), "›`¨");
    assert_eq!(converter.convert("অনিন্দ্য"), "Awb›`¨");
}

#[test]
fn test_140() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্দ্ব"), "›Ø");
    assert_eq!(converter.convert("দ্বন্দ্ব"), "Ø›Ø");
}

#[test]
fn test_141() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্দ্র"), "›`ª");
    assert_eq!(converter.convert("কেন্দ্র"), "†K›`ª");
}

#[test]
fn test_142() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ধ"), "Ü");
    assert_eq!(converter.convert("অন্ধ"), "AÜ");
}

#[test]
fn test_143() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ধ্য"), "Ü¨");
    assert_eq!(converter.convert("বিন্ধ্য"), "weÜ¨");
}

#[test]
fn test_144() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ধ্র"), "Üª");
    assert_eq!(converter.convert("রন্ধ্র"), "iÜª");
}

#[test]
fn test_145() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ন"), "bœ");
    assert_eq!(converter.convert("নবান্ন"), "bevbœ");
}

#[test]
fn test_146() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ব"), "š^");
    assert_eq!(converter.convert("ধন্বন্তরি"), "aš^š—wi");
}

#[test]
fn test_147() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্ম"), "b¥");
    assert_eq!(converter.convert("চিন্ময়"), "wPb¥q");
}

#[test]
fn test_148() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ন্য"), "b¨");
    assert_eq!(converter.convert("ধন্য"), "ab¨");
    assert_eq!(converter.convert("ধান্য"), "avb¨");
}

#[test]
fn test_149() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্ট"), "Þ");
    assert_eq!(converter.convert("পাটি-সাপ্টা"), "cvwU-mvÞv");
    assert_eq!(converter.convert("ক্যাপ্টেন"), "K¨v‡Þb");
}

#[test]
fn test_150() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্ত"), "ß");
    assert_eq!(converter.convert("সুপ্ত"), "myß");
}

#[test]
fn test_151() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্ন"), "cœ");
    assert_eq!(converter.convert("স্বপ্ন"), "¯^cœ");
}

#[test]
fn test_152() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্প"), "à");
    assert_eq!(converter.convert("ধাপ্পা"), "avàv");
}

#[test]
fn test_153() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্য"), "c¨");
    assert_eq!(converter.convert("প্রাপ্য"), "cÖvc¨");
}

#[test]
fn test_154() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্র"), "cÖ");
    assert_eq!(converter.convert("ক্ষিপ্র"), "w¶cÖ");
}

#[test]
fn test_155() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্র্য"), "cÖ¨");
    assert_eq!(converter.convert("প্র্যাকটিস"), "cÖ¨vKwUm");
}

#[test]
fn test_156() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্ল"), "c­");
    assert_eq!(converter.convert("আপ্লুত"), "Avc­“Z");
}

#[test]
fn test_157() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("প্স"), "á");
    assert_eq!(converter.convert("লিপ্সা"), "wjáv");
}

#[test]
fn test_158() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ফ্র"), "d«");
    assert_eq!(converter.convert("ফ্রক"), "d«K");
    assert_eq!(converter.convert("ফ্রিজ"), "wd«R");
    assert_eq!(converter.convert("আফ্রিকা"), "Avwd«Kv");
    assert_eq!(converter.convert("রেফ্রিজারেটর"), "†iwd«Rv‡iUi");
}

#[test]
fn test_159() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ফ্ল"), "d¬");
    assert_eq!(converter.convert("ফ্লেভার"), "†d¬fvi");
}

#[test]
fn test_160() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্জ"), "â");
    assert_eq!(converter.convert("ন্যুব্জ"), "b¨yâ");
}

#[test]
fn test_161() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্দ"), "ã");
    assert_eq!(converter.convert("জব্দ"), "Rã");
    assert_eq!(converter.convert("শব্দ"), "kã");
}

#[test]
fn test_162() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্ধ"), "ä");
    assert_eq!(converter.convert("লব্ধ"), "jä");
}

#[test]
fn test_163() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্ব"), "eŸ");
    assert_eq!(converter.convert("ডাব্বা"), "WveŸv");
}

#[test]
fn test_164() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্য"), "e¨");
    assert_eq!(converter.convert("দাতব্য"), "`vZe¨");
    assert_eq!(converter.convert("কর্তব্য"), "KZ©e¨");
}

#[test]
fn test_165() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্র"), "eª");
    assert_eq!(converter.convert("ব্রাহ্মণ"), "eªvþY");
}

#[test]
fn test_166() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ব্ল"), "e­");
    assert_eq!(converter.convert("ব্লাউজ"), "e­vDR");
}

#[test]
fn test_167() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ভ্ব"), "f¡");
    assert_eq!(converter.convert("ভ্বা"), "f¡v");
}

#[test]
fn test_168() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ভ্য"), "f¨");
    assert_eq!(converter.convert("সভ্য"), "mf¨");
}

#[test]
fn test_169() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ভ্র"), "å");
    assert_eq!(converter.convert("শুভ্র"), "ïå");
    assert_eq!(converter.convert("অভ্র"), "Aå");
}

#[test]
fn test_170() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ভ্ল"), "f¬");
    assert_eq!(converter.convert("ভ্লাদিমি"), "f¬vw`wg");
}

#[test]
fn test_171() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ন"), "æ");
    assert_eq!(converter.convert("নিম্ন"), "wbæ");
}

#[test]
fn test_172() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্প"), "¤c");
    assert_eq!(converter.convert("কম্প"), "K¤c");
    assert_eq!(converter.convert("শম্পা"), "k¤cv");
}

#[test]
fn test_173() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্প্র"), "¤cÖ");
    assert_eq!(converter.convert("সম্প্রতি"), "m¤cÖwZ");
}

#[test]
fn test_174() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ফ"), "ç");
    assert_eq!(converter.convert("লম্ফ"), "jç");
}

#[test]
fn test_175() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ব"), "¤^");
    assert_eq!(converter.convert("প্রতিবিম্ব"), "cÖwZwe¤^");
    assert_eq!(converter.convert("অম্বর"), "A¤^i");
}

#[test]
fn test_176() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ব্র"), "¤^ª");
    assert_eq!(converter.convert("মেম্ব্রেন"), "†g‡¤^ªb");
}

#[test]
fn test_177() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ভ"), "¤¢");
    assert_eq!(converter.convert("দম্ভ"), "`¤¢");
}

#[test]
fn test_178() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ভ্র"), "¤£");
    assert_eq!(converter.convert("সম্ভ্রম"), "m¤£g");
}

#[test]
fn test_179() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ম"), "¤§");
    assert_eq!(converter.convert("সম্মান"), "m¤§vb");
}

#[test]
fn test_180() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্য"), "g¨");
    assert_eq!(converter.convert("গ্রাম্য"), "MÖvg¨");
}

#[test]
fn test_181() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্র"), "gª");
    assert_eq!(converter.convert("নম্র"), "bgª");
}

#[test]
fn test_182() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ম্ল"), "¤¬");
    assert_eq!(converter.convert("অম্ল"), "A¤¬");
}

#[test]
fn test_183() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("য্য"), "h¨");
    assert_eq!(converter.convert("ন্যায্য"), "b¨vh¨");
}

#[test]
fn test_184() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ক"), "K©");
    assert_eq!(converter.convert("তর্ক"), "ZK©");
}

#[test]
fn test_185() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ক্য"), "K¨©");
    assert_eq!(converter.convert("অতর্ক্য"), "AZK¨©");
}

#[test]
fn test_186() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্গ"), "M©");
    assert_eq!(converter.convert("বর্গ"), "eM©");
}

#[test]
fn test_187() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্গ্য"), "M¨©");
    assert_eq!(converter.convert("বর্গ্য"), "eM¨©");
}

#[test]
fn test_188() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ঘ্য"), "N¨©");
    assert_eq!(converter.convert("দৈর্ঘ্য"), "ˆ`N¨©");
}

#[test]
fn test_189() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ঙ্গ"), "½©");
    assert_eq!(converter.convert("শার্ঙ্গ"), "kv½©");
}

#[test]
fn test_190() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্চ্য"), "P¨©");
    assert_eq!(converter.convert("অর্চ্য"), "AP¨©");
}

#[test]
fn test_191() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্জ্য"), "R¨©");
    assert_eq!(converter.convert("বর্জ্য"), "eR¨©");
}

#[test]
fn test_192() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্জ্জ"), "¾©");
    assert_eq!(converter.convert("ঊর্জ্জ"), "E¾©");
}

#[test]
fn test_193() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্জ্ঞ"), "Á©");
    assert_eq!(converter.convert("দুর্জ্ঞেয়"), "`y‡Á©q");
}

#[test]
fn test_194() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ণ্য"), "Y¨©");
    assert_eq!(converter.convert("বৈবর্ণ্য"), "ˆeeY¨©");
}

#[test]
fn test_195() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ত্য"), "Z¨©");
    assert_eq!(converter.convert("মর্ত্য"), "gZ¨©");
}

#[test]
fn test_196() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্থ্য"), "_¨©");
    assert_eq!(converter.convert("সামর্থ্য"), "mvg_¨©");
}

#[test]
fn test_197() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ব্য"), "e¨©");
    assert_eq!(converter.convert("নৈর্ব্যক্তিক"), "ˆbe¨©w³K");
}

#[test]
fn test_198() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ম্য"), "g¨©");
    assert_eq!(converter.convert("নৈষ্কর্ম্য"), "ˆb®‹g¨©");
}

#[test]
fn test_199() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্শ্য"), "k¨©");
    assert_eq!(converter.convert("অস্পর্শ্য"), "A¯ck¨©");
}

#[test]
fn test_200() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ষ্য"), "l¨©");
    assert_eq!(converter.convert("ঔৎকর্ষ্য"), "JrKl¨©");
}

#[test]
fn test_201() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্হ্য"), "n¨©");
    assert_eq!(converter.convert("গর্হ্য"), "Mn¨©");
}

#[test]
fn test_202() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্খ"), "L©");
    assert_eq!(converter.convert("মূর্খ"), "g~L©");
}

#[test]
fn test_203() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্গ"), "M©");
    assert_eq!(converter.convert("দুর্গ"), "`yM©");
}

#[test]
fn test_204() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্গ্র"), "MÖ©");
    assert_eq!(converter.convert("দুর্গ্রহ"), "`yMÖ©n");
    assert_eq!(converter.convert("নির্গ্রন্হ"), "wbMÖ©›n");
}

#[test]
fn test_205() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ঘ"), "N©");
    assert_eq!(converter.convert("দীর্ঘ"), "`xN©");
}

#[test]
fn test_206() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্চ"), "P©");
    assert_eq!(converter.convert("অর্চনা"), "AP©bv");
}

#[test]
fn test_207() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ছ"), "Q©");
    assert_eq!(converter.convert("মূর্ছনা"), "g~Q©bv");
}

#[test]
fn test_208() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্জ"), "R©");
    assert_eq!(converter.convert("অর্জন"), "AR©b");
}

#[test]
fn test_209() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ঝ"), "S©");
    assert_eq!(converter.convert("নির্ঝর"), "wbS©i");
}

#[test]
fn test_210() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ট"), "U©");
    assert_eq!(converter.convert("আর্ট"), "AvU©");
    assert_eq!(converter.convert("কোর্ট"), "†KvU©");
    assert_eq!(converter.convert("কম্ফর্টার"), "KçU©vi");
    assert_eq!(converter.convert("শার্ট"), "kvU©");
    assert_eq!(converter.convert("কার্টিজ"), "KvwU©R");
    assert_eq!(converter.convert("আর্টিস্ট"), "AvwU©÷");
    assert_eq!(converter.convert("পোর্টম্যানটো"), "†cvU©g¨vb‡Uv");
    assert_eq!(converter.convert("সার্টিফিকেট"), "mvwU©wd‡KU");
    assert_eq!(converter.convert("কনসার্ট"), "KbmvU©");
    assert_eq!(converter.convert("কার্টুন"), "KvU©zb");
    assert_eq!(converter.convert("কোয়ার্টার"), "†KvqvU©vi");
}

#[test]
fn test_211() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ড"), "W©");
    assert_eq!(converter.convert("অর্ডার"), "AW©vi");
    assert_eq!(converter.convert("লর্ড"), "jW©");
    assert_eq!(converter.convert("বর্ডার"), "eW©vi");
    assert_eq!(converter.convert("কার্ড"), "KvW©");
}

#[test]
fn test_212() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ণ"), "Y©");
    assert_eq!(converter.convert("বর্ণ"), "eY©");
}

#[test]
fn test_213() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ত"), "Z©");
    assert_eq!(converter.convert("ক্ষুধার্ত"), "¶zavZ©");
}

#[test]
fn test_214() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ত্ম"), "Í©");
    assert_eq!(converter.convert("ক্লিন্নবর্ত্মাস্থি"), "wK¬bœeÍ©vw¯’");
}

#[test]
fn test_215() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ত্র"), "Î©");
    assert_eq!(converter.convert("কর্ত্রী"), "KÎ©x");
}

#[test]
fn test_216() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্থ"), "_©");
    assert_eq!(converter.convert("অর্থ"), "A_©");
}

#[test]
fn test_217() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্দ"), "`©");
    assert_eq!(converter.convert("নির্দয়"), "wb`©q");
}

#[test]
fn test_218() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্দ্ব"), "Ø©");
    assert_eq!(converter.convert("নির্দ্বিধা"), "wbwØ©av");
}

#[test]
fn test_219() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্দ্র"), "`ª©");
    assert_eq!(converter.convert("আর্দ্র"), "Av`ª©");
}

#[test]
fn test_220() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ধ"), "a©");
    assert_eq!(converter.convert("গোলার্ধ"), "†Mvjva©");
}

#[test]
fn test_221() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ধ্ব"), "aŸ©");
    assert_eq!(converter.convert("ঊর্ধ্ব"), "EaŸ©");
}

#[test]
fn test_222() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ন"), "b©");
    assert_eq!(converter.convert("দুর্নাম"), "`yb©vg");
}

#[test]
fn test_223() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্প"), "c©");
    assert_eq!(converter.convert("দর্প"), "`c©");
}

#[test]
fn test_224() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ফ"), "d©");
    assert_eq!(converter.convert("স্কার্ফ"), "¯‹vd©");
}

#[test]
fn test_225() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ব"), "e©");
    assert_eq!(converter.convert("উর্বর"), "De©i");
}

#[test]
fn test_226() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ভ"), "f©");
    assert_eq!(converter.convert("গর্ভ"), "Mf©");
}

#[test]
fn test_227() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ম"), "g©");
    assert_eq!(converter.convert("ধর্ম"), "ag©");
}

#[test]
fn test_228() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্য"), "h©");
    assert_eq!(converter.convert("আর্য"), "Avh©");
}

#[test]
fn test_229() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ল"), "j©");
    assert_eq!(converter.convert("দুর্লভ"), "`yj©f");
}

#[test]
fn test_230() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্শ"), "k©");
    assert_eq!(converter.convert("স্পর্শ"), "¯ck©");
}

#[test]
fn test_231() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্শ্ব"), "k¦©");
    assert_eq!(converter.convert("পার্শ্ব"), "cvk¦©");
}

#[test]
fn test_232() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ষ"), "l©");
    assert_eq!(converter.convert("ঘর্ষণ"), "Nl©Y");
    assert_eq!(converter.convert("ধর্ষণ"), "al©Y");
}

#[test]
fn test_233() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ষ্ট"), "ó©");
    assert_eq!(converter.convert("ধার্ষ্টামি"), "avó©vwg");
}

#[test]
fn test_234() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ষ্ণ"), "ò©");
    assert_eq!(converter.convert("পার্ষ্ণিকাস্থি"), "cvwò©Kvw¯’");
}

#[test]
fn test_235() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ষ্ণ্য"), "ò¨©");
    assert_eq!(converter.convert("কার্ষ্ণ্য"), "Kvò¨©");
}

#[test]
fn test_236() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্স"), "m©");
    assert_eq!(converter.convert("জার্সি"), "Rvwm©");
    assert_eq!(converter.convert("নার্স"), "bvm©");
    assert_eq!(converter.convert("পার্সেল"), "cv‡m©j");
    assert_eq!(converter.convert("কুর্সি"), "Kzwm©");
}

#[test]
fn test_237() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্হ"), "n©");
    assert_eq!(converter.convert("গার্হস্থ্য"), "Mvn©¯’¨");
}

#[test]
fn test_238() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্হ্য"), "n¨©");
    assert_eq!(converter.convert("গর্হ্য"), "Mn¨©");
}

#[test]
fn test_239() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("র্ঢ্য"), "X¨©");
    assert_eq!(converter.convert("দার্ঢ্য"), "`vX¨©");
}

#[test]
fn test_240() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ক"), "é");
    assert_eq!(converter.convert("শুল্ক"), "ïé");
}

#[test]
fn test_241() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ক্য"), "é¨");
    assert_eq!(converter.convert("যাজ্ঞবল্ক্য"), "hvÁeé¨");
}

#[test]
fn test_242() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্গ"), "ê");
    assert_eq!(converter.convert("বল্গা"), "eêv");
}

#[test]
fn test_243() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ট"), "ë");
    assert_eq!(converter.convert("উল্টো"), "D‡ëv");
}

#[test]
fn test_244() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ড"), "ì");
    assert_eq!(converter.convert("ফিল্ডিং"), "wdwìs");
}

#[test]
fn test_245() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্প"), "í");
    assert_eq!(converter.convert("বিকল্প"), "weKí");
}

#[test]
fn test_246() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ফ"), "î");
    assert_eq!(converter.convert("গুল্ফ"), "¸î");
}

#[test]
fn test_247() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ব"), "j¡");
    assert_eq!(converter.convert("বিল্ব"), "wej¡");
    assert_eq!(converter.convert("বাল্ব"), "evj¡");
}

#[test]
fn test_248() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ভ"), "j¢");
    assert_eq!(converter.convert("প্রগল্ভ"), "cÖMj¢");
}

#[test]
fn test_249() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ম"), "j¥");
    assert_eq!(converter.convert("গুল্ম"), "¸j¥");
}

#[test]
fn test_250() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্য"), "j¨");
    assert_eq!(converter.convert("তারল্য"), "Zvij¨");
}

#[test]
fn test_251() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ল্ল"), "j−");
    assert_eq!(converter.convert("উল্লাস"), "Dj−vm");
}

#[test]
fn test_252() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্চ"), "ð");
    assert_eq!(converter.convert("পুনশ্চ"), "cybð");
}

#[test]
fn test_253() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্ছ"), "ñ");
    assert_eq!(converter.convert("শিরশ্ছেদ"), "wki‡ñ`");
}

#[test]
fn test_254() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্ন"), "kœ");
    assert_eq!(converter.convert("প্রশ্ন"), "cÖkœ");
}

#[test]
fn test_255() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্ব"), "k¦");
    assert_eq!(converter.convert("বিশ্ব"), "wek¦");
    assert_eq!(converter.convert("অশ্ব"), "Ak¦");
}

#[test]
fn test_256() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্ম"), "k¥");
    assert_eq!(converter.convert("জীবাশ্ম"), "Rxevk¥");
}

#[test]
fn test_257() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্য"), "k¨");
    assert_eq!(converter.convert("অবশ্য"), "Aek¨");
}

#[test]
fn test_258() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্র"), "kª");
    assert_eq!(converter.convert("মিশ্র"), "wgkª");
}

#[test]
fn test_259() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("শ্ল"), "k−");
    assert_eq!(converter.convert("অশ্লীল"), "Ak−xj");
    assert_eq!(converter.convert("শ্লোক"), "†k−vK");
}

#[test]
fn test_260() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ক"), "®‹");
    assert_eq!(converter.convert("শুষ্ক"), "ï®‹");
}

#[test]
fn test_261() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ক্ব"), "®‹¡");
    assert_eq!(converter.convert("নিষ্ক্বাথ"), "wb®‹¡v_");
}

#[test]
fn test_262() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ক্র"), "®Œ");
    assert_eq!(converter.convert("নিষ্ক্রিয়"), "wbw®Œq");
}

#[test]
fn test_263() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ট"), "ó");
    assert_eq!(converter.convert("কষ্ট"), "Kó");
}

#[test]
fn test_264() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ট্য"), "ó¨");
    assert_eq!(converter.convert("বৈশিষ্ট্য"), "ˆewkó¨");
}

#[test]
fn test_265() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ট্র"), "óª");
    assert_eq!(converter.convert("রাষ্ট্র"), "ivóª");
}

#[test]
fn test_266() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ঠ"), "ô");
    assert_eq!(converter.convert("শ্রেষ্ঠ"), "†kªô");
}

#[test]
fn test_267() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ঠ্য"), "ô¨");
    assert_eq!(converter.convert("নিষ্ঠ্যূত"), "wbô¨~Z");
}

#[test]
fn test_268() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ণ"), "ò");
    assert_eq!(converter.convert("কৃষ্ণ"), "K…ò");
}

#[test]
fn test_269() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ণ্ব"), "ò¡");
    assert_eq!(converter.convert("বিষ্ণ্বিন্দ্র"), "wewò¡›`ª");
}

#[test]
fn test_270() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্প"), "®c");
    assert_eq!(converter.convert("নিষ্পাপ"), "wb®cvc");
}

#[test]
fn test_271() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্প্র"), "®cÖ");
    assert_eq!(converter.convert("নিষ্প্রয়োজন"), "wb®cÖ‡qvRb");
}

#[test]
fn test_272() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ফ"), "õ");
    assert_eq!(converter.convert("নিষ্ফল"), "wbõj");
}

#[test]
fn test_273() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ব"), "®^");
    assert_eq!(converter.convert("মাতৃষ্বসা"), "gvZ…®^mv");
}

#[test]
fn test_274() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্ম"), "®§");
    assert_eq!(converter.convert("উষ্ম"), "D®§");
}

#[test]
fn test_275() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("ষ্য"), "l¨");
    assert_eq!(converter.convert("শিষ্য"), "wkl¨");
}

#[test]
fn test_276() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ক"), "¯‹");
    assert_eq!(converter.convert("মনোস্কামনা"), "g‡bv¯‹vgbv");
}

#[test]
fn test_277() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ক্র"), "¯Œ");
    assert_eq!(converter.convert("ইস্ক্রু"), "B¯Œy");
}

#[test]
fn test_278() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্খ"), "ö");
    assert_eq!(converter.convert("স্খলন"), "öjb");
}

#[test]
fn test_279() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ট"), "÷");
    assert_eq!(converter.convert("স্টেশন"), "†÷kb");
}

#[test]
fn test_280() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ট্র"), "÷ª");
    assert_eq!(converter.convert("স্ট্রাইক"), "÷ªvBK");
}

#[test]
fn test_281() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ত"), "¯—");
    assert_eq!(converter.convert("ব্যস্ত"), "e¨¯—");
    assert_eq!(converter.convert("ন্যস্ত"), "b¨¯—");
}

#[test]
fn test_282() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ত্ব"), "¯—¡");
    assert_eq!(converter.convert("বহিস্ত্বক"), "ewn¯—¡K");
}

#[test]
fn test_283() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ত্য"), "¯—¨");
    assert_eq!(converter.convert("অস্ত্যর্থ"), "A¯—¨_©");
}

#[test]
fn test_284() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ত্র"), "¯¿");
    assert_eq!(converter.convert("স্ত্রী"), "¯¿x");
}

#[test]
fn test_285() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্থ"), "¯’");
    assert_eq!(converter.convert("দুঃস্থ"), "`yt¯’");
}

#[test]
fn test_286() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্থ্য"), "¯’¨");
    assert_eq!(converter.convert("স্বাস্থ্য"), "¯^v¯’¨");
}

#[test]
fn test_287() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ন"), "ø");
    assert_eq!(converter.convert("স্নান"), "øvb");
}

#[test]
fn test_288() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ন্য"), "ø¨");
}

#[test]
fn test_289() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্প"), "¯c");
    assert_eq!(converter.convert("আস্পর্ধা"), "Av¯ca©v");
}

#[test]
fn test_290() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্প্র"), "¯cÖ");
    assert_eq!(converter.convert("স্প্রিং"), "w¯cÖs");
}

#[test]
fn test_291() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্প্ল"), "¯c−");
    assert_eq!(converter.convert("স্প্লিন"), "w¯c−b");
}

#[test]
fn test_292() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ফ"), "ù");
    assert_eq!(converter.convert("আস্ফালন"), "Avùvjb");
}

#[test]
fn test_293() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ব"), "¯^");
    assert_eq!(converter.convert("স্বর"), "¯^i");
}

#[test]
fn test_294() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ম"), "¯§");
    assert_eq!(converter.convert("স্মরণ"), "¯§iY");
}

#[test]
fn test_295() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্য"), "m¨");
    assert_eq!(converter.convert("শস্য"), "km¨");
}

#[test]
fn test_296() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্র"), "mª");
    assert_eq!(converter.convert("অজস্র"), "ARmª");
}

#[test]
fn test_297() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("স্ল"), "¯¬");
    assert_eq!(converter.convert("স্লোগান"), "†¯¬vMvb");
}

#[test]
fn test_298() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্ণ"), "nœ");
    assert_eq!(converter.convert("অপরাহ্ণ"), "Acivnœ");
}

#[test]
fn test_299() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্ন"), "ý");
    assert_eq!(converter.convert("চিহ্ন"), "wPý");
}

#[test]
fn test_300() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্ব"), "nŸ");
    assert_eq!(converter.convert("আহ্বান"), "AvnŸvb");
}

#[test]
fn test_301() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্ম"), "þ");
    assert_eq!(converter.convert("ব্রাহ্মণ"), "eªvþY");
}

#[test]
fn test_302() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্য"), "n¨");
    assert_eq!(converter.convert("বাহ্য"), "evn¨");
    assert_eq!(converter.convert("সহ্য"), "mn¨");
}

#[test]
fn test_303() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্র"), "nª");
    assert_eq!(converter.convert("হ্রদ"), "nª`");
}

#[test]
fn test_304() {
    let converter = Bijoy2000::new();
    assert_eq!(converter.convert("হ্ল"), "n¬");
    assert_eq!(converter.convert("আহ্লাদ"), "Avn¬v`");
    assert_eq!(converter.convert("প্রহ্লাদ"), "cÖn¬v`");
}

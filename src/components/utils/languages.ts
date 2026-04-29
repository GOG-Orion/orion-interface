import languages from './languages.json';

export interface LanguageEntry {
    lang: string;
    menu_head: string;
    menu_item01: string;
    menu_item02: string;
    menu_item03: string;
    menu_item04: string;
    int_head: string;
    int_search: string;
}

export function getMenuText(language: string) {
    const selectedLanguage = languages.find((lang) => lang.lang === language) as LanguageEntry | undefined;
    const english = languages.find((lang) => lang.lang === "ENG") as LanguageEntry | undefined;

    return selectedLanguage || english || (languages[0] as LanguageEntry);
}

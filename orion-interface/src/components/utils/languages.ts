import languages from './languages.json'; // Adjust the path accordingly

export function getMenuText(language: string) {
    const selectedLanguage = languages.find(lang => lang.lang === language);
    if (selectedLanguage) {
        return selectedLanguage;
    } else {
        return languages[0]; // Default to English if language not found
    }
}
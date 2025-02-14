import { createContext, useContext, useState } from "react";
import languages from "./utils/languages.json";


interface LanguageContextType {
  selectedLanguage: string;
  menuText: Record<string, string>;
  setLanguage: (lang: string) => void;
}

const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

export const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const [selectedLanguage, setSelectedLanguage] = useState("ENG");

  const getMenuText = (lang: string) => {
    return languages.find((l) => l.lang === lang) || languages[0]; 
  };

  const [menuText, setMenuText] = useState(getMenuText(selectedLanguage));

  const setLanguage = (lang: string) => {
    setSelectedLanguage(lang);
    setMenuText(getMenuText(lang)); 
  };

  return (
    <LanguageContext.Provider value={{ selectedLanguage, menuText, setLanguage }}>
      {children}
    </LanguageContext.Provider>
  );
};

export const useLanguage = () => {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error("useLanguage must be used within a LanguageProvider");
  }
  return context;
};

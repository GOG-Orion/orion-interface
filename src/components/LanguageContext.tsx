import { createContext, useContext, useState, type ReactNode } from "react";
import { getMenuText, type LanguageEntry } from "./utils/languages";


interface LanguageContextType {
  selectedLanguage: string;
  menuText: LanguageEntry;
  setLanguage: (lang: string) => void;
}


const LanguageContext = createContext<LanguageContextType | undefined>(undefined);


export const LanguageProvider = ({ children }: { children: ReactNode }) => {
  const [selectedLanguage, setSelectedLanguage] = useState("ENG");
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

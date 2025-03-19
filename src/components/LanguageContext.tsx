import { createContext, useContext, useState } from "react";
import languages from "./utils/languages.json";

// Definindo a estrutura do contexto
interface LanguageContextType {
  selectedLanguage: string;
  menuText: Record<string, string>;
  setLanguage: (lang: string) => void;
}

// Criando o contexto
const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

// Provedor do contexto
export const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const [selectedLanguage, setSelectedLanguage] = useState("ENG");

  // Função para buscar os textos no JSON
  const getMenuText = (lang: string) => {
    return languages.find((l) => l.lang === lang) || languages[0]; // Retorna o idioma selecionado ou o primeiro idioma como fallback
  };

  // Estado que mantém os textos do idioma atual
  const [menuText, setMenuText] = useState(getMenuText(selectedLanguage));

  // Função para alterar o idioma
  const setLanguage = (lang: string) => {
    setSelectedLanguage(lang);
    setMenuText(getMenuText(lang)); // Atualiza os textos sempre que o idioma muda
  };

  return (
    <LanguageContext.Provider value={{ selectedLanguage, menuText, setLanguage }}>
      {children}
    </LanguageContext.Provider>
  );
};

// Hook para acessar o contexto
export const useLanguage = () => {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error("useLanguage must be used within a LanguageProvider");
  }
  return context;
};

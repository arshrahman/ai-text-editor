# AI Text Editor  

A **Notion-like** rich text editor with AI-powered text summarization, paraphrasing, and content generation.  

---

## 🚀 Features  
- **Rich Text Editing** – A seamless, Notion-style writing experience.  
- **AI-powered Assistance** – Summarize, expand, or paraphrase content

---

## 🛠 Tech Stack  
- **Frontend:** TypeScript – Next.js  
- **Backend:** Rust – Axum  

---

## 📌 API Details  
### **Endpoint:**  
```http
POST /ai/action
```  
### **Payload:**  
```json
{
  "content": "your text content",
  "command": "summarize" | "expand" | "paraphrase"
}
```  

---

## 🔧 Setup & Run  

### **Frontend**  
```sh
pnpm install && pnpm dev
```  

### **Backend**  
1. **Install Dotenvx:**  
   ```sh
   curl -fsS https://dotenvx.sh | sh
   ```  
2. **Add your Gemini API key** to `.env`.  
3. **Encrypt the .env file:**  
   ```sh
   dotenvx encrypt
   ```  
   ⚠️ **Do not push `.env.keys` to source control.**  
4. **Run the backend with decrypted env variables:**  
   ```sh
   dotenvx run cargo run
   ```  

---

🚀 **You're ready to build with AI-powered text editing!**


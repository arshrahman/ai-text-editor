import { Editor } from "@/components/dynamic-editor";

export default function Home() {
  return (
    <main className="bg-background text-foreground">
      <div className="container mx-auto p-24">
        <Editor />
      </div>
    </main>
  );
}

"use client"; // this registers <Editor> as a Client Component
import "@blocknote/core/fonts/inter.css";
import { useCreateBlockNote } from "@blocknote/react";
import { BlockNoteView } from "@blocknote/mantine";
import "@blocknote/mantine/style.css";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useState } from "react";

// Our <Editor> component we can reuse later
export default function Editor() {
  // Creates a new editor instance.
  const editor = useCreateBlockNote();
  const [markdown, setMarkdown] = useState<string>("");
  const [operation, setOperation] = useState("summarize");
  const [loading, setLoading] = useState(false);

  const handleAskAI = async () => {
    try {
      setLoading(true);
      const response = await fetch('http://localhost:3000/api/ai', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: markdown,
          operation: operation,
        }),
      });

      const data = await response.json();
      if (!response.ok) {
        throw new Error(data.error || 'Failed to process text');
      }

      // Update editor content with the result
      const blocks = await editor.tryParseMarkdownToBlocks(data.result);
      editor.replaceBlocks(editor.document, blocks);
      
    } catch (error) {
      console.error('Error processing text:', error);
      // You might want to add proper error handling here
    } finally {
      setLoading(false);
    }
  };

  const onChange = async () => {
    // Converts the editor's contents from Block objects to Markdown and store to state.
    const markdown = await editor.blocksToMarkdownLossy(editor.document);
    setMarkdown(markdown);
  };

  // Renders the editor instance using a React component.
  return (
    <div className="flex flex-col gap-4">
      <BlockNoteView theme='light' editor={editor} onChange={onChange} className="h-[50vh] border border-gray-300 rounded-sm p-4" />
      <div className="flex justify-end items-center gap-2">
        <Select value={operation} onValueChange={setOperation}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Select operation" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="paraphrase">Paraphrase</SelectItem>
            <SelectItem value="summarize">Summarize</SelectItem>
            <SelectItem value="expand">Expand</SelectItem>
          </SelectContent>
        </Select>
        <Button onClick={handleAskAI} disabled={loading}>
          {loading ? 'Processing...' : 'Ask AI'}
        </Button>
      </div>
    </div>
  );
}
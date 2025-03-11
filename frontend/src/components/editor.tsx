"use client";
import "@blocknote/core/fonts/inter.css";
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
import { useEditor } from "@/hooks/useEditor";
import { Command } from "@/services/api";
import { FormattingToolbarController, FormattingToolbar, BlockTypeSelect, FileCaptionButton, FileReplaceButton, BasicTextStyleButton, TextAlignButton, ColorStyleButton, NestBlockButton, UnnestBlockButton, CreateLinkButton } from "@blocknote/react";
import { AiDropdown } from "./ui/ai-dropdown";

// Our <Editor> component we can reuse later
export default function Editor() {
  const {
    editor,
    operation,
    loading,
    error,
    setOperation,
    processContent
  } = useEditor();

  const handleAiOperation = async (selectedOperation: Command) => {
    setOperation(selectedOperation);
    await processContent({onlySelected: true});
  };

  // Renders the editor instance using a React component.
  return (
    <div className="flex flex-col gap-4">
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
          <span className="block sm:inline">{error}</span>
        </div>
      )}
      <BlockNoteView 
        theme='light' 
        editor={editor} 
        className="h-[50vh] overflow-y-scroll border border-gray-300 rounded-sm p-4" 
        formattingToolbar={false}>
        <FormattingToolbarController
        formattingToolbar={() => (
          <FormattingToolbar>
            <BlockTypeSelect key={"blockTypeSelect"} />

            <AiDropdown onSelect={handleAiOperation} />

            <FileCaptionButton key={"fileCaptionButton"} />
            <FileReplaceButton key={"replaceFileButton"} />

            <BasicTextStyleButton
              basicTextStyle={"bold"}
              key={"boldStyleButton"}
            />
            <BasicTextStyleButton
              basicTextStyle={"italic"}
              key={"italicStyleButton"}
            />
            <BasicTextStyleButton
              basicTextStyle={"underline"}
              key={"underlineStyleButton"}
            />
            <BasicTextStyleButton
              basicTextStyle={"strike"}
              key={"strikeStyleButton"}
            />
            {/* Extra button to toggle code styles */}
            <BasicTextStyleButton
              key={"codeStyleButton"}
              basicTextStyle={"code"}
            />

            <TextAlignButton
              textAlignment={"left"}
              key={"textAlignLeftButton"}
            />
            <TextAlignButton
              textAlignment={"center"}
              key={"textAlignCenterButton"}
            />
            <TextAlignButton
              textAlignment={"right"}
              key={"textAlignRightButton"}
            />

            <ColorStyleButton key={"colorStyleButton"} />

            <NestBlockButton key={"nestBlockButton"} />
            <UnnestBlockButton key={"unnestBlockButton"} />

            <CreateLinkButton key={"createLinkButton"} />
          </FormattingToolbar>
        )}
        />
      </BlockNoteView>
      <div className="flex justify-end items-center gap-2">
        <Select value={operation} onValueChange={(value: Command) => setOperation(value)}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Select operation" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="paraphrase">Paraphrase</SelectItem>
            <SelectItem value="summarize">Summarize</SelectItem>
            <SelectItem value="expand">Expand</SelectItem>
          </SelectContent>
        </Select>
        <Button onClick={() => processContent()} disabled={loading}>
          {loading ? 'Processing...' : 'Ask AI'}
        </Button>
      </div>
    </div>
  );
}
import { useState } from 'react';
import { useCreateBlockNote } from '@blocknote/react';
import { aiService, APIError, Command } from '@/services/api';

export const useEditor = () => {
  const editor = useCreateBlockNote();
  const [operation, setOperation] = useState<Command>("summarize");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const processContent = async ({onlySelected}: {onlySelected?: boolean} = {}) => {
    try {
      setLoading(true);
      setError(null);

    const blocksToProcess = onlySelected ? editor.getSelection()?.blocks : editor.document;
    if (!blocksToProcess) return;

    const content = await editor.blocksToMarkdownLossy(blocksToProcess);
    const response = await aiService.processText({
        content,
        command: operation,
      });

      const blocks = await editor.tryParseMarkdownToBlocks(response.result);
      editor.replaceBlocks(blocksToProcess, blocks);
    } catch (err) {
      const errorMessage = err instanceof APIError
        ? err.message
        : 'An unexpected error occurred';
      setError(errorMessage);
      console.error('Error processing text:', err);
    } finally {
      setLoading(false);
    }
  };

  return {
    editor,
    operation,
    loading,
    error,
    setOperation,
    processContent,
  };
};
// API service for handling all AI-related requests

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export type Command = "summarize" | "paraphrase" | "expand";

export interface AIRequestPayload {
  content: string;
  command: Command;
}

export interface AIResponse {
  result: string;
  command: string;
}

export class APIError extends Error {
  constructor(public statusCode: number, message: string) {
    super(message);
    this.name = 'APIError';
  }
}

export const aiService = {
  async processText(payload: AIRequestPayload): Promise<AIResponse> {
    const response = await fetch(`${API_URL}/ai/action`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new APIError(response.status, data.error || 'Failed to process text');
    }

    return data;
  },
};
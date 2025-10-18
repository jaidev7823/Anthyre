import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { useState, useEffect, useRef } from "react";

interface Message {
  id: number;
  text: string;
  sender: "user" | "bot";
}

const Chat = () => {
  const [messages, setMessages] = useState<Message[]>([
    { id: 1, text: "Hello! How can I help you today?", sender: "bot" },
  ]);
  const [inputValue, setInputValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSendMessage = () => {
    if (inputValue.trim() === "") return;

    const userMessage: Message = {
      id: messages.length + 1,
      text: inputValue,
      sender: "user",
    };
    setMessages((prevMessages) => [...prevMessages, userMessage]);
    setInputValue("");

    // Simulate bot response with animation
    const botResponseText = "I am working";
    const botResponse: Message = {
      id: messages.length + 2,
      text: "",
      sender: "bot",
    };
    setMessages((prevMessages) => [...prevMessages, botResponse]);

    const words = botResponseText.split(" ");
    let currentText = "";
    words.forEach((word, index) => {
      setTimeout(() => {
        currentText += (index > 0 ? " " : "") + word;
        setMessages((prevMessages) =>
          prevMessages.map((msg) =>
            msg.id === botResponse.id ? { ...msg, text: currentText } : msg
          )
        );
      }, (index + 1) * 300);
    });
  };

  return (
    <div className="flex flex-col h-full">
      <Card className="flex-grow bg-transparent border-0 shadow-none">
        <CardHeader>
          <CardTitle>Chat</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4 h-[calc(100vh-200px)] overflow-y-auto">
          {messages.map((message) => (
            <div
              key={message.id}
              className={`flex items-start gap-4 ${
                message.sender === "user" ? "justify-end" : ""
              }`}
            >
              {message.sender === "bot" && (
                <div className="flex-shrink-0 w-10 h-10 rounded-full bg-muted" />
              )}
              <div
                className={`p-3 rounded-lg ${
                  message.sender === "user"
                    ? "bg-primary text-primary-foreground"
                    : "bg-muted"
                }`}
              >
                <p>{message.text}</p>
              </div>
              {message.sender === "user" && (
                <div className="flex-shrink-0 w-10 h-10 rounded-full bg-muted" />
              )}
            </div>
          ))}
          <div ref={messagesEndRef} />
        </CardContent>
        <CardFooter>
          <div className="flex w-full space-x-2">
            <Input
              placeholder="Type your message..."
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
            />
            <Button onClick={handleSendMessage}>Send</Button>
          </div>
        </CardFooter>
      </Card>
    </div>
  );
};

export default Chat;
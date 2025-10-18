import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

const Chat = () => {
  return (
    <div className="flex flex-col h-full">
      <Card className="flex-grow">
        <CardHeader>
          <CardTitle>Chat</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-start gap-4">
            <div className="flex-shrink-0 w-10 h-10 rounded-full bg-muted" />
            <div className="p-3 rounded-lg bg-muted">
              <p>Hello! How can I help you today?</p>
            </div>
          </div>
          <div className="flex items-start gap-4 justify-end">
            <div className="p-3 rounded-lg bg-primary text-primary-foreground">
              <p>I need help with my time management.</p>
            </div>
            <div className="flex-shrink-0 w-10 h-10 rounded-full bg-muted" />
          </div>
        </CardContent>
        <CardFooter>
          <div className="flex w-full space-x-2">
            <Input placeholder="Type your message..." />
            <Button>Send</Button>
          </div>
        </CardFooter>
      </Card>
    </div>
  );
};

export default Chat;

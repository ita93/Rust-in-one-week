pub struct Queue {
    older : Vec<char>, //older elements, eldest last.
    younger: Vec<char>, //younger elements, youngest last.
}

impl Queue {
    //push a character onto the back of a queue
    push fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    //pop a character off the front of a queue, 
    //Return Some(c) if there was a character to pop
    //or None if the queue was empty
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty(){
            if self.younger.is_empty{
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reserve();
        }
        self.older.pop();
    }
}

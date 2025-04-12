struct Node
{
    int data;
    Node *next;
    Node(int val, Node *nxt = nullptr) : data(val), next(nxt) {};
};

class MyLinkedList
{
private:
    Node *head;

public:
    MyLinkedList() : head(nullptr) {}

    int get(int index)
    {
        int count = 0;
        Node *temp = head;
        while (temp != nullptr)
        {
            if (count == index)
                return temp->data;
            temp = temp->next;
            count++;
        }
        return -1;
    }

    void addAtHead(int val)
    {
        Node *newNode = new Node(val);
        newNode->next = head;
        head = newNode;
    }

    void addAtTail(int val)
    {
        Node *newNode = new Node(val);
        if (head == nullptr)
        {
            head = newNode;
            return;
        }

        Node *temp = head;
        while (temp->next != nullptr)
        {
            temp = temp->next;
        }
        temp->next = newNode;
    }

    void addAtIndex(int index, int val)
    {
        if (index == 0)
        {
            addAtHead(val);
            return;
        }

        int count = 0;
        Node *temp = head;
        while (temp != nullptr && count < index - 1)
        {
            temp = temp->next;
            count++;
        }

        if (temp == nullptr) return;

        Node *newNode = new Node(val);
        newNode->next = temp->next;
        temp->next = newNode;
    }

    void deleteAtIndex(int index)
    {
        if (head == nullptr) return;

        if (index == 0)
        {
            Node *temp = head;
            head = head->next;
            delete temp;
            return;
        }

        int count = 0;
        Node *temp = head;
        while (temp != nullptr && count < index - 1)
        {
            temp = temp->next;
            count++;
        }

        if (temp == nullptr || temp->next == nullptr) return;

        Node *delNode = temp->next;
        temp->next = delNode->next;
        delete delNode;
    }
};


/**
 * Your MyLinkedList object will be instantiated and called as such:
 * MyLinkedList* obj = new MyLinkedList();
 * int param_1 = obj->get(index);
 * obj->addAtHead(val);
 * obj->addAtTail(val);
 * obj->addAtIndex(index,val);
 * obj->deleteAtIndex(index);
 */
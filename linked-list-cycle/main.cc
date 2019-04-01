#include <iostream>

/*Definition for singly-linked list.*/
struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x)
      : val(x)
      , next(NULL)
  {
  }
};

class Solution {
  public:
  bool hasCycle(ListNode* head)
  {
    ListNode *p = head, *q = head;
    do {
      if (q == nullptr)
        return false;
      p = p->next;
      if (q->next)
        q = q->next->next;
      else
        return false;
    } while (p != q);
    return true;
  }
};

int main(void)
{
  ListNode* l = new ListNode(0);
  for (int i = 1; i < 3; ++i) {
    l->next = new ListNode(i);
    l = l->next;
  }
  return 0;
}

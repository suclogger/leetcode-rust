fn main() {
    println!("Hello, world!");
}
/*这题不支持rust编译，所以用java写了，知道思路就很简单的
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public ListNode detectCycle(ListNode head) {
        ListNode slow = head;
        ListNode fast = head;
        do {
            if(fast == null || fast.next == null) {
                return null;
            }
            slow = slow.next;
            fast = fast.next.next;
        } while (fast != slow);
        fast = head;
        while(fast != slow) {
            slow = slow.next;
            fast = fast.next;
        }
        return fast;
    }
}

*/
#[cfg(test)]
#[test]
fn should_be_fifo() {
    use crate::common::ds::queue::Queue;

    let mut q: Queue<i32> = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), None);
}

#[test]
fn should_have_accurate_count() {
    use crate::common::ds::queue::Queue;

    let mut q: Queue<()> = Queue::new();

    assert_eq!(q.count(), 0);
    q.enqueue(());
    assert_eq!(q.count(), 1);
    q.enqueue(());
    assert_eq!(q.count(), 2);
    q.enqueue(());
    assert_eq!(q.count(), 3);

    q.dequeue();
    assert_eq!(q.count(), 2);
    q.dequeue();
    assert_eq!(q.count(), 1);
    q.dequeue();
    assert_eq!(q.count(), 0);
    q.dequeue();
    assert_eq!(q.count(), 0);
}

#[test]
fn should_be_empty_iff_count_eq_zero() {
    use crate::common::ds::queue::Queue;

    let mut q: Queue<()> = Queue::new();

    assert_eq!(q.is_empty(), true);
    q.enqueue(());
    assert_eq!(q.is_empty(), false);
    q.enqueue(());
    assert_eq!(q.is_empty(), false);

    q.dequeue();
    assert_eq!(q.is_empty(), false);
    q.dequeue();
    assert_eq!(q.is_empty(), true);
    q.dequeue();
    assert_eq!(q.is_empty(), true);
}

#[test]
fn should_be_able_to_peek_next() {
    use crate::common::ds::queue::Queue;

    let mut q: Queue<i32> = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    assert_eq!(q.peek(), Some(&1));
    assert_eq!(q.peek(), Some(&1));
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.peek(), Some(&2));
    q.dequeue();
    q.dequeue();
    assert_eq!(q.peek(), None);
}

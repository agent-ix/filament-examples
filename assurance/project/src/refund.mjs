// A refund is accepted when its amount is at most the amount paid.
export function refund(order, amount) {
  if (amount > order.paid) return { accepted: false };
  return { accepted: true, amount };
}

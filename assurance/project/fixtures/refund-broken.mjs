// A change that breaks the refund rule: every refund is accepted.
export function refund(order, amount) {
  return { accepted: true, amount };
}

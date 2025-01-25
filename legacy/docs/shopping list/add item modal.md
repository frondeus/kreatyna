# Add Item Modal Components Structure

## Core Components Needed

| Component          | Category   | Notes                                                                | Progress   |
| ------------------ | ---------- | -------------------------------------------------------------------- | ---------- |
| Component          | Category   | Notes                                                                | Progress   |
| -----------        | ---------- | --------                                                             | ---------- |
| `ActionIconButton` | atom       | Reusable icon button component - can replace with Mantine component  | x          |
| `FloatingButton`   | atom       | Fixed position button component - can replace with Mantine component | x          |
| `ProposalCheckbox` | atom       | Checkbox with states for proposals                                   | x          |
| `ProposalUnit`     | atom       | Unit display/edit component                                          | x          |
| `ItemProposal`     | molecule   | Shopping list item proposal component                                |            |
| `AddItemModal`     | organism   | Main modal component                                                 |            |
| `EditUnitDrawer`   | organism   | Bottom drawer for editing amounts/units                              |            |

## Features & Behaviors

### Search & Add Items

- Input field at top for searching/adding items
- Autofocus on input when modal opens
- Real-time filtering of existing items as user types
- Enter key submits the current input

### Item Proposals

- Shows matching existing items while typing
- Creates new item proposal if no match found
- Displays list of all previous items below
- Each proposal shows:
  - Add/increment button (+ icon)
  - Item name
  - Amount (if > 0)
  - Unit (if set)
  - Decrease/remove button (- or x icon)

### Item States

- Items can be in states:
  - TODO (default for new items)
  - IN_CART
  - NOT_BUYING
  - BOUGHT

### Amount & Unit Management

- Click + icon to add/increment item
- Click - icon to decrease amount
- Click x icon to remove when amount is 1
- Click amount/unit to open edit drawer
- Edit drawer allows:
  - Change amount via number input
  - Add/edit/remove unit
  - Common units like 'g' available

### UI Interactions

- Floating save button appears when:
  - Changes are made to existing items
  - New item text is entered
- Item name shows full text in popup on click if truncated
- Amount/unit becomes editable only when:
  - Amount > 0
  - Item is not in BOUGHT state
- Visual feedback on state changes:
  - - icon fills when item added
  - Error color for remove buttons
  - Disabled states when item bought

### Edge Cases

- Empty state handling
- Long item names truncation
- Large numbers formatting
- Unit validation
- Duplicate item prevention
- State transition rules
- Mobile-friendly interactions

## Data Structure

```typescript
interface ItemProposal {
  id: string;
  name: string;
  amount: number;
  unit?: string;
  state: "TODO" | "IN_CART" | "NOT_BUYING" | "BOUGHT";
}
```

## Key Differences from Legacy

The React implementation should maintain all functionality but:

- Use Mantine components instead of Skeleton UI
- Follow React patterns vs Svelte
- Maintain consistent styling with rest of v2 app
- Use React context for state management
- TypeScript interfaces instead of Svelte types

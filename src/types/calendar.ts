export interface Calendar {
  id: string;
  name: string;
  color: string;
  created_at: Date;
  updated_at: Date;
  is_primary: boolean;
}

export interface CalendarViewType {
  value: 'month' | 'week' | 'day';
  label: string;
}

export interface Calendar {
  id: string;
  name: string;
  color: string;
  createdAt: Date;
  updatedAt: Date;
  isPrimary: boolean;
}

export interface CalendarViewType {
  value: 'month' | 'week' | 'day';
  label: string;
}
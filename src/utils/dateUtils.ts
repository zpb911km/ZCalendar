export const dateUtils = {
  // 日期格式化
  formatLocal(date: Date, formatStr: string = 'yyyy-MM-dd HH:mm:ss'): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');

    return formatStr
      .replace('yyyy', year.toString())
      .replace('MM', month)
      .replace('dd', day)
      .replace('HH', hours)
      .replace('mm', minutes)
      .replace('ss', seconds);
  },

  // 获取月视图的日期数组
  getMonthDates(date: Date): Date[] {
    const monthStart = this.startOfMonth(date);
    const monthEnd = this.endOfMonth(monthStart);
    const weekStart = this.startOfWeek(monthStart);
    const weekEnd = this.endOfWeek(monthEnd);

    const dates: Date[] = [];
    let current = weekStart;

    while (current <= weekEnd) {
      dates.push(new Date(current));
      current = this.addDays(current, 1);
    }

    return dates;
  },

  // 获取周视图的日期数组
  getWeekDates(date: Date): Date[] {
    const weekStart = this.startOfWeek(date);
    const dates: Date[] = [];

    for (let i = 0; i < 7; i++) {
      dates.push(this.addDays(weekStart, i));
    }

    return dates;
  },

  // 获取某天的开始时间
  startOfDay(date: Date): Date {
    return new Date(
      date.getFullYear(),
      date.getMonth(),
      date.getDate(),
      0,
      0,
      0,
      0
    );
  },

  // 获取月份的开始日期
  startOfMonth(date: Date): Date {
    return new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0, 0);
  },

  // 获取月份的结束日期
  endOfMonth(date: Date): Date {
    return new Date(
      date.getFullYear(),
      date.getMonth() + 1,
      0,
      23,
      59,
      59,
      999
    );
  },

  // 获取周的开始日期（周日）
  startOfWeek(date: Date): Date {
    const dayOfWeek = date.getDay(); // 0 (Sunday) to 6 (Saturday)
    const start = new Date(date);
    start.setDate(date.getDate() - dayOfWeek);
    return this.startOfDay(start);
  },

  // 获取周的结束日期（周六）
  endOfWeek(date: Date): Date {
    const dayOfWeek = date.getDay(); // 0 (Sunday) to 6 (Saturday)
    const end = new Date(date);
    end.setDate(date.getDate() + (6 - dayOfWeek));
    return this.endOfDay(end);
  },

  // 获取日期的结束时间
  endOfDay(date: Date): Date {
    return new Date(
      date.getFullYear(),
      date.getMonth(),
      date.getDate(),
      23,
      59,
      59,
      999
    );
  },

  // 检查两个日期是否是同一个月
  isSameMonth(date1: Date, date2: Date): boolean {
    return (
      date1.getFullYear() === date2.getFullYear() &&
      date1.getMonth() === date2.getMonth()
    );
  },

  // 检查两个日期是否是同一天
  isSameDay(date1: Date, date2: Date): boolean {
    return (
      date1.getFullYear() === date2.getFullYear() &&
      date1.getMonth() === date2.getMonth() &&
      date1.getDate() === date2.getDate()
    );
  },

  // 日期加减操作
  addMonths(date: Date, months: number): Date {
    const result = new Date(date);
    result.setMonth(result.getMonth() + months);
    return result;
  },

  subMonths(date: Date, months: number): Date {
    const result = new Date(date);
    result.setMonth(result.getMonth() - months);
    return result;
  },

  addWeeks(date: Date, weeks: number): Date {
    const result = new Date(date);
    result.setDate(result.getDate() + weeks * 7);
    return result;
  },

  subWeeks(date: Date, weeks: number): Date {
    const result = new Date(date);
    result.setDate(result.getDate() - weeks * 7);
    return result;
  },

  addDays(date: Date, days: number): Date {
    const result = new Date(date);
    result.setDate(result.getDate() + days);
    return result;
  },

  // 日期解析
  parseISODateString(dateString: string): Date {
    return new Date(dateString);
  },

  // 获取月份的名称
  getMonthName(date: Date): string {
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    return `${year}年${month}月`;
  },

  // 获取星期的名称
  getWeekdayName(date: Date): string {
    const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
    return days[date.getDay()];
  },

  // 获取日期的数字
  getDayNumber(date: Date): string {
    return date.getDate().toString();
  },
};

// RFC5545兼容的日期格式化函数
export const icalDateUtils = {
  // 格式化为iCalendar日期格式
  formatIcalDate(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}${month}${day}`;
  },

  // 格式化为iCalendar日期时间格式
  formatIcalDateTime(date: Date): string {
    const year = date.getUTCFullYear();
    const month = String(date.getUTCMonth() + 1).padStart(2, '0');
    const day = String(date.getUTCDate()).padStart(2, '0');
    const hours = String(date.getUTCHours()).padStart(2, '0');
    const minutes = String(date.getUTCMinutes()).padStart(2, '0');
    const seconds = String(date.getUTCSeconds()).padStart(2, '0');
    return `${year}${month}${day}T${hours}${minutes}${seconds}Z`;
  },

  // 将iCalendar格式解析为Date对象
  parseIcalDate(dateStr: string): Date {
    if (dateStr.length === 8) {
      // 日期格式 (YYYYMMDD)
      const year = parseInt(dateStr.substring(0, 4));
      const month = parseInt(dateStr.substring(4, 6)) - 1; // 月份从0开始
      const day = parseInt(dateStr.substring(6, 8));
      return new Date(year, month, day, 0, 0, 0, 0);
    } else if (dateStr.length === 15 || dateStr.length === 16) {
      // 日期时间格式 (YYYYMMDDTHHMMSSZ)
      const cleanStr = dateStr.replace('T', '').replace('Z', '');
      const year = parseInt(cleanStr.substring(0, 4));
      const month = parseInt(cleanStr.substring(4, 6)) - 1; // 月份从0开始
      const day = parseInt(cleanStr.substring(6, 8));
      const hour = parseInt(cleanStr.substring(9, 11));
      const minute = parseInt(cleanStr.substring(11, 13));
      const second = parseInt(cleanStr.substring(13, 15));
      const date = new Date(Date.UTC(year, month, day, hour, minute, second));
      return date;
    } else {
      throw new Error(`Invalid iCalendar date format: ${dateStr}`);
    }
  },
};

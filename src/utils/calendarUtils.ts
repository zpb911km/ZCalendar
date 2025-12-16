export const calendarUtils = {
  // 计算两个日期之间的天数差
  daysBetween(date1: Date, date2: Date): number {
    const oneDay = 24 * 60 * 60 * 1000; // hours*minutes*seconds*milliseconds
    const diffDays = Math.round(Math.abs((date1.getTime() - date2.getTime()) / oneDay));
    return diffDays;
  },
  
  // 检查日期是否在指定范围内
  isDateInRange(date: Date, startDate: Date, endDate: Date): boolean {
    return date >= startDate && date <= endDate;
  },
  
  // 检查两个时间段是否重叠
  isTimeOverlap(start1: Date, end1: Date, start2: Date, end2: Date): boolean {
    return start1 < end2 && start2 < end1;
  },
  
  // 获取时间块（小时）
  getTimeBlocks(startHour: number = 0, endHour: number = 24, interval: number = 1): number[] {
    const blocks: number[] = [];
    for (let hour = startHour; hour < endHour; hour += interval) {
      blocks.push(hour);
    }
    return blocks;
  },
  
  // 将时间转换为显示格式
  formatTimeForDisplay(date: Date): string {
    return date.toLocaleTimeString('zh-CN', { 
      hour: '2-digit', 
      minute: '2-digit',
      hour12: false 
    });
  },
  
  // 生成重复事件实例
  generateRecurringEvents(baseEvent: any, recurrenceRule: string, startDate: Date, endDate: Date): any[] {
    // 这里需要根据RRULE生成重复事件
    // 简化实现，实际应用中需要完整的RRULE解析
    const events: any[] = [];
    
    // 基本实现：如果事件有重复规则，生成实例
    if (recurrenceRule) {
      // 解析RRULE并生成重复事件
      // 这里只是一个占位实现
      const ruleParts = recurrenceRule.split(';');
      let frequency: string | undefined;
      let interval: number = 1;
      let count: number | undefined;
      
      for (const part of ruleParts) {
        const [key, value] = part.split('=');
        switch (key) {
          case 'FREQ':
            frequency = value;
            break;
          case 'INTERVAL':
            interval = parseInt(value) || 1;
            break;
          case 'COUNT':
            count = parseInt(value);
            break;
        }
      }
      
      if (frequency) {
        // 生成重复实例
        let currentDate = new Date(baseEvent.start);
        let occurrenceCount = 0;
        
        while (currentDate <= endDate && (count === undefined || occurrenceCount < count)) {
          if (currentDate >= startDate) {
            const eventInstance = {
              ...baseEvent,
              id: `${baseEvent.id}_${occurrenceCount}`,
              start: new Date(currentDate),
              end: new Date(currentDate.getTime() + (baseEvent.end.getTime() - baseEvent.start.getTime())),
              recurrenceId: currentDate.toISOString()
            };
            events.push(eventInstance);
            occurrenceCount++;
          }
          
          // 根据频率递增日期
          switch (frequency) {
            case 'DAILY':
              currentDate.setDate(currentDate.getDate() + interval);
              break;
            case 'WEEKLY':
              currentDate.setDate(currentDate.getDate() + 7 * interval);
              break;
            case 'MONTHLY':
              currentDate.setMonth(currentDate.getMonth() + interval);
              break;
            case 'YEARLY':
              currentDate.setFullYear(currentDate.getFullYear() + interval);
              break;
            default:
              // 不支持的频率，跳出循环
              currentDate = endDate;
          }
        }
      }
    }
    
    return events;
  }
};
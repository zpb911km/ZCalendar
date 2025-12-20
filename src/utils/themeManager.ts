/**
 * 主题管理器
 * 统一管理应用的主题和个性化设置
 */

interface ThemeSettings {
  theme: 'light' | 'dark' | 'auto';
  primaryColor: string;
}

class ThemeManager {
  private static instance: ThemeManager;
  private settings: ThemeSettings = {
    theme: 'auto',
    primaryColor: '#007bff',
  };

  private constructor() {
    this.loadSettings();
  }

  public static getInstance(): ThemeManager {
    if (!ThemeManager.instance) {
      ThemeManager.instance = new ThemeManager();
    }
    return ThemeManager.instance;
  }

  /**
   * 从 localStorage 加载设置
   */
  public loadSettings(): void {
    const savedTheme =
      (localStorage.getItem('theme') as 'light' | 'dark' | 'auto') || 'auto';
    const savedPrimaryColor = localStorage.getItem('primaryColor') || '#007bff';

    this.settings = {
      theme: savedTheme,
      primaryColor: savedPrimaryColor,
    };

    this.applySettings();
  }

  /**
   * 保存设置到 localStorage
   */
  public saveSettings(): void {
    localStorage.setItem('theme', this.settings.theme);
    localStorage.setItem('primaryColor', this.settings.primaryColor);
  }

  /**
   * 应用当前设置
   */
  public applySettings(): void {
    this.applyTheme();
    this.applyPrimaryColor();
  }

  /**
   * 应用主题 (浅色/深色)
   */
  private applyTheme(): void {
    const root = document.documentElement;
    const prefersDark = window.matchMedia(
      '(prefers-color-scheme: dark)'
    ).matches;

    if (
      this.settings.theme === 'dark' ||
      (this.settings.theme === 'auto' && prefersDark)
    ) {
      root.setAttribute('data-theme', 'dark');
    } else {
      root.setAttribute('data-theme', 'light');
    }
  }

  /**
   * 应用主色调
   */
  private applyPrimaryColor(): void {
    const root = document.documentElement;
    root.style.setProperty('--primary-color', this.settings.primaryColor);

    // 更新相关的颜色变体
    this.updatePrimaryColorVariants();
  }

  /**
   * 根据主色调更新相关的颜色变体 (深色、浅色版本)
   */
  private updatePrimaryColorVariants(): void {
    const root = document.documentElement;
    const primaryColor = this.settings.primaryColor;

    // 直接设置主色调
    root.style.setProperty('--primary-color', primaryColor);

    // 从主色调派生其他颜色
    // 计算深色版本 (降低亮度)
    const darkerColor = this.adjustBrightness(primaryColor, -30);
    root.style.setProperty('--primary-dark', darkerColor);

    // 计算浅色版本 (提高亮度，增加透明度效果)
    const lighterColor = this.adjustBrightness(primaryColor, 60);
    root.style.setProperty('--primary-light', lighterColor);
  }

  /**
   * 将十六进制颜色转换为 RGB
   */
  // private hexToRgb(hex: string): { r: number; g: number; b: number } | null {
  //   const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  //   return result ? {
  //     r: parseInt(result[1], 16),
  //     g: parseInt(result[2], 16),
  //     b: parseInt(result[1], 16)
  //   } : null;
  // }

  /**
   * 调整颜色亮度
   */
  private adjustBrightness(hex: string, percent: number): string {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    if (!result) return hex;

    const r = Math.min(255, Math.max(0, parseInt(result[1], 16) + percent));
    const g = Math.min(255, Math.max(0, parseInt(result[2], 16) + percent));
    const b = Math.min(255, Math.max(0, parseInt(result[3], 16) + percent));

    return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
  }

  /**
   * 设置主题模式
   */
  public setTheme(theme: 'light' | 'dark' | 'auto'): void {
    this.settings.theme = theme;
    this.applyTheme();
    this.saveSettings();
  }

  /**
   * 设置主色调
   */
  public setPrimaryColor(color: string): void {
    this.settings.primaryColor = color;
    this.applyPrimaryColor();
    this.saveSettings();
  }

  /**
   * 获取当前设置
   */
  public getSettings(): ThemeSettings {
    return { ...this.settings };
  }

  /**
   * 监听系统主题变化
   */
  public watchSystemThemeChange(): void {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', () => {
      if (this.settings.theme === 'auto') {
        this.applyTheme();
      }
    });
  }
}

export const themeManager = ThemeManager.getInstance();

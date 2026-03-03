let loadingBarInstance: any = null;

export const setLoadingBarInstance = (instance: any) => {
  loadingBarInstance = instance;
};

export const useLoading = () => {
  const showLoading = () => {
    if (loadingBarInstance) {
      loadingBarInstance.show();
    }
  };

  const hideLoading = () => {
    if (loadingBarInstance) {
      loadingBarInstance.hide();
    }
  };

  const withLoading = async <T>(fn: () => Promise<T>): Promise<T> => {
    try {
      showLoading();
      const result = await fn();
      return result;
    } finally {
      hideLoading();
    }
  };

  return {
    showLoading,
    hideLoading,
    withLoading,
  };
};

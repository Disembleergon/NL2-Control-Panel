export const PORT = 4641;
export const HOSTNAME_STORAGE_KEY = "nl2-control-panel-hostname";

export async function getBase64FromUrl(url: string) {
  const data = await fetch(url);
  const blob = await data.blob();
  return new Promise((resolve) => {
    const reader = new FileReader();
    reader.readAsDataURL(blob);
    reader.onloadend = () => {
      const base64data = reader.result;
      resolve(base64data);
    };
  });
}

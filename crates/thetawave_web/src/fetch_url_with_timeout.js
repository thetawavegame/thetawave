/**
 * Fetches data from the specified URL with a timeout. Prefers logging to throwing an error.
 * @param {string} url - The URL to fetch data from.
 * @param {number} timeoutMs - The timeout duration in milliseconds.
 * @returns {Promise<string>} A Promise that resolves to the fetched data as a string.
 */
export async function fetchWithTimeout(url, timeoutMs) {
  // Modeled after https://stackoverflow.com/a/50101022
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(url, { signal: controller.signal });
    clearTimeout(timeoutId);
    console.log(response);
    if (!response.ok) {
      console.error(
        `Network response was not ok: ${response.status} ${response.statusText}`,
      );
      return;
    }
    const res = await response.text();
    return res;
  } catch (err) {
    if (err.name == "AbortError") {
      console.error("Failed to download url in time.");
      return;
    }
    console.error("Uncaught error fetching data js", err);
  }
}

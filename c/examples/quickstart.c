/* Quickstart: build with `cc quickstart.c $(pkg-config --cflags --libs narwhalapi)`.
 * Reads NARWHAL_API_KEY (and optionally NARWHAL_BASE_URL) from the environment. */
#include <stdio.h>

#include <narwhalapi.h>

int main(void) {
    if (!nw_version_check(NW_VERSION_MAJOR, NW_VERSION_MINOR)) {
        fprintf(stderr, "libnarwhalapi %s does not match this header\n", nw_version());
        return 1;
    }
    nw_client *client = NULL;
    nw_error *err = NULL;
    if (nw_client_new(NULL, &client, &err) != NW_OK) {
        fprintf(stderr, "%s\n", nw_error_message(err));
        nw_error_free(err);
        return 1;
    }

    nw_economics_cpi_params params = {0};
    params.size = sizeof params; /* lets older programs run against newer libraries */
    nw_cpi *cpi = NULL; /* no period: the latest release */
    if (nw_economics_cpi(client, "USA", &params, &cpi, &err) != NW_OK) {
        fprintf(stderr, "%s\n", nw_error_message(err));
        nw_error_free(err);
        nw_client_free(client);
        return 1;
    }
    /* Decimals are exact wire strings; never convert them to floating point. */
    printf("CPI %s %s = %s\n", cpi->country, cpi->period, cpi->value);
    nw_cpi_free(cpi);
    nw_client_free(client);
    return 0;
}

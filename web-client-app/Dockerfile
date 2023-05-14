# Use the official Apache HTTP Server base image
FROM httpd:latest

# Copy the index.html file to the Apache document root
COPY index.html /usr/local/apache2/htdocs/

# Enable the mod_include module
RUN sed -i '/mod_include.so/s/^#//g' /usr/local/apache2/conf/httpd.conf

# Add SSI directives to Apache configuration
RUN echo "AddType text/html .html" >> /usr/local/apache2/conf/httpd.conf \
    && echo "AddOutputFilter INCLUDES .html" >> /usr/local/apache2/conf/httpd.conf \
    && echo "Options +Includes" >> /usr/local/apache2/conf/httpd.conf

# Expose port 80
EXPOSE 3000

# Start the Apache HTTP Server
CMD ["httpd-foreground"]
